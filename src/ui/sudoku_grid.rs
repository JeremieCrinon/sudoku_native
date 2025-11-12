use crate::resolver::{Grid, fill_grid, solve_grid};
use iced::{
    Element, Pixels, Task,
    widget::{
        Column, Container, Row, Text, button, column, container, row, scrollable, text, text_input,
    },
};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio;

pub struct SudokuGrid {
    value: Grid,
    error: Option<String>,
    json_result: Option<String>,
    json_progress: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArrayGrid([[u8; 9]; 9]);

#[derive(Debug, Clone)]
pub enum SudokuMessage {
    InputChanged {
        x: usize,
        y: usize,
        value: String,
    },
    Solve,
    Reset,
    LoadJson,
    JsonFinished {
        solved_grids: Vec<ArrayGrid>,
        duration: Duration,
    },
    JsonError {
        error: String,
    },
}

async fn handle_solving(grid_to_solve: ArrayGrid) -> ArrayGrid {
    // We make a new grid object, this object will be easier to manipulate than if we used the 2D array directly
    let mut grid = Grid::new();

    // We call the function fill_grid from /sudoku/fill.rs with the grid object, and the grid_to_solve that the JS gave us
    grid = fill_grid(grid, grid_to_solve.0);

    // We call the solve_grid function, it returns two elements, the solved grid if it can be solved, and success (true = the grid has been solved and is correct, false = the grid cannot be solved and isn't correct)
    let (grid, success) = solve_grid(grid);

    // If success, we make result the grid object that we turned back into an array, else, we return an empty grid, the JS will then know that the grid isn't correct
    let result = if success {
        grid.to_array()
    } else {
        [[0; 9]; 9]
    };

    ArrayGrid(result)
}

impl SudokuGrid {
    pub fn new() -> (Self, Task<SudokuMessage>) {
        (
            SudokuGrid {
                value: Grid::new(),
                error: None,
                json_result: None,
                json_progress: 0.0 as f32,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: SudokuMessage) -> Task<SudokuMessage> {
        // Reset the error when there is an update, as it will not be relevent anymore
        self.error = None;

        match message {
            SudokuMessage::InputChanged { x, y, value } => {
                // Parse the input to a number an make it 0 if it's not valid
                let value_trimmed = value.trim();

                let value_parsed = if !value_trimmed.is_empty() && value_trimmed.len() == 1 {
                    value_trimmed
                        .chars()
                        .next()
                        .and_then(|c| c.to_digit(10).map(|d| d as u8))
                } else {
                    None
                };

                if let Some(num) = value_parsed {
                    self.value.set(x, y, num);
                } else {
                    self.value.set(x, y, 0);
                }

                Task::none()
            }
            SudokuMessage::Solve => {
                // We call the solve_grid function, it returns two elements, the solved grid if it can be solved, and success (true = the grid has been solved and is correct, false = the grid cannot be solved and isn't correct)
                let (grid, success) = solve_grid(self.value.clone());

                if success {
                    self.value = grid;
                } else {
                    self.error = Some("The grid is not solvable".to_string());
                }

                Task::none()
            }
            SudokuMessage::Reset => {
                self.value = Grid::new();
                Task::none()
            }
            SudokuMessage::LoadJson => {
                self.json_result = Some("Solving, please wait...".to_string()); // Reset the json resolving results
                self.json_progress = 0.0; // Set the progress bar back to 0

                // Use Task::perform to run async work
                Task::perform(
                    async {
                        let file_path = FileDialog::new()
                            .add_filter("text", &["json"])
                            .set_directory("/")
                            .pick_file();

                        let start_total = Instant::now(); // Get the current time (for calculating how much time it took)

                        let file_path = match file_path {
                            Some(path) => path,
                            None => {
                                return Err("No file selected".to_string());
                            }
                        };

                        // Read the file content as bytes
                        let file_content = match std::fs::read(&file_path) {
                            Ok(content) => content,
                            Err(_) => {
                                return Err("Failed to read file".to_string());
                            }
                        };

                        // Parse as UTF-8 string
                        let file_str = match std::str::from_utf8(&file_content) {
                            Ok(s) => s,
                            Err(_) => {
                                return Err("File is not valid UTF-8".to_string());
                            }
                        };

                        // Parse JSON
                        let grids: Vec<ArrayGrid> = match serde_json::from_str(file_str) {
                            Ok(grids) => grids,
                            Err(_) => {
                                return Err("The file is not valid JSON".to_string());
                            }
                        };

                        // Spawn all solving tasks
                        let solves: Vec<_> = grids
                            .iter()
                            .map(|grid| {
                                let grid = grid.clone();
                                tokio::spawn(async move { handle_solving(grid).await })
                            })
                            .collect();

                        // Wait for all tasks to complete and collect results
                        let mut solved_grids: Vec<ArrayGrid> = Vec::with_capacity(grids.len());
                        for handle in solves {
                            match handle.await {
                                Ok(solved_grid) => solved_grids.push(solved_grid),
                                Err(e) => eprintln!("Task failed: {:?}", e),
                            }
                        }

                        let duration = start_total.elapsed();

                        Ok((solved_grids, duration))
                    },
                    |result| match result {
                        Ok((solved_grids, duration)) => SudokuMessage::JsonFinished {
                            solved_grids,
                            duration,
                        },
                        Err(error) => SudokuMessage::JsonError { error: error },
                    },
                )
            }
            SudokuMessage::JsonFinished {
                solved_grids,
                duration,
            } => {
                // Ask user where to save the file
                let save_path = FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_file_name("solved_grids.json")
                    .save_file();

                if let Some(path) = save_path {
                    // Serialize and save the solved grids
                    match serde_json::to_string_pretty(&solved_grids) {
                        Ok(json_string) => match std::fs::write(&path, json_string) {
                            Ok(_) => {}
                            Err(e) => {
                                self.error = Some(format!("Failed to save file: {}", e));
                            }
                        },
                        Err(e) => {
                            self.error = Some(format!("Failed to serialize results: {}", e));
                        }
                    }
                }

                self.json_result = Some(format!(
                    "
                    Number of grids : {}.
                    Time spent : {:.2?},
                    Average time per grid : {:.2?}
                    ",
                    solved_grids.len(),
                    duration,
                    duration / solved_grids.len() as u32
                ));

                Task::none()
            }
            SudokuMessage::JsonError { error } => {
                self.error = Some(error);

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, SudokuMessage> {
        let rows = (0..9).map(|y| {
            let cells = (0..9).map(|x| {
                // Convert the value to string and make it empty if it's 0 (0 = empty cell)
                let value = self.value.get(x, y);
                let value_displayed = if value != 0 {
                    value.to_string()
                } else {
                    String::new()
                };

                text_input("", &value_displayed)
                    .on_input({
                        let x = x;
                        let y = y;
                        move |v| SudokuMessage::InputChanged { x, y, value: v }
                    })
                    .width(75)
                    .size(Pixels(55.0))
                    .padding(0)
                    .into()
            });

            row(cells).into()
        });

        let grid: Column<SudokuMessage> = column(rows).into();

        let solve_button: Container<SudokuMessage> = container(
            button("Solve the grid")
                .style(button::primary)
                .on_press(SudokuMessage::Solve),
        )
        .padding(10)
        .into();

        let reset_button: Container<SudokuMessage> = container(
            button("Reset the grid")
                .style(button::secondary)
                .on_press(SudokuMessage::Reset),
        )
        .padding(10)
        .into();

        let load_json_button: Container<SudokuMessage> = container(
            button("Load a json file")
                .style(button::primary)
                .on_press(SudokuMessage::LoadJson),
        )
        .padding(10)
        .into();

        let buttons: Row<SudokuMessage> = row![solve_button, reset_button];

        let error_display: Text = text({
            match &self.error {
                Some(err) => err.as_str(),
                None => "",
            }
        })
        .style(text::danger);

        let json_results_display: Text = text({
            match &self.json_result {
                Some(r) => r.as_str(),
                None => "",
            }
        });

        scrollable(column![
            error_display,
            grid,
            buttons,
            load_json_button,
            json_results_display
        ])
        .into()
    }
}
