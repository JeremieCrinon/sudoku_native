use crate::resolver::{Grid, fill_grid, solve_grid};
use iced::{
    Element, Pixels,
    widget::{
        Column, Container, ProgressBar, Row, Text, button, column, container, progress_bar, row,
        scrollable, text, text_input,
    },
};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tokio;

#[derive(Default)]
pub struct SudokuGrid {
    value: Grid,
    error: Option<String>,
    json_result: Option<String>,
    json_progress: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ArrayGrid([[u8; 9]; 9]);

#[derive(Debug, Clone)]
pub enum SudokuMessage {
    InputChanged { x: usize, y: usize, value: String },
    Solve,
    Reset,
    LoadJson,
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

    // // Add the grid we just solved to our array of solved grids
    // solved_grids.push(ArrayGrid(result));
    //
    // // Update the progress bar. For now, there is no async support, so the
    // // rendering will be blocked during solving and the progress bar will not be
    // // updated
    // self.json_progress =
    //     (grids.iter().count() as f32 / solved_grids.iter().count() as f32) * 100 as f32;
    //
    // println!(
    //     "Just solved grid {} out of {}",
    //     &solved_grids.iter().count(),
    //     &grids.iter().count()
    // );
}

impl SudokuGrid {
    pub fn update(&mut self, message: SudokuMessage) {
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
            }
            SudokuMessage::Solve => {
                // We call the solve_grid function, it returns two elements, the solved grid if it can be solved, and success (true = the grid has been solved and is correct, false = the grid cannot be solved and isn't correct)
                let (grid, success) = solve_grid(self.value.clone());

                if success {
                    self.value = grid;
                } else {
                    self.error = Some("The grid is not solvable".to_string());
                }
            }
            SudokuMessage::Reset => {
                self.value = Grid::new();
            }
            SudokuMessage::LoadJson => {
                self.json_result = None; // Reset the json resolving results
                self.json_progress = 0.0; // Set the progress bar back to 0

                let file_path = FileDialog::new()
                    .add_filter("text", &["json"])
                    .set_directory("/")
                    .pick_file();

                let start_total = Instant::now(); // Get the current time (for calculating how much time it took)

                let file_path = match file_path {
                    Some(path) => path,
                    None => {
                        self.error = Some("No file selected".to_string());
                        return;
                    }
                };

                // Read the file content as bytes
                let file_content = match std::fs::read(&file_path) {
                    Ok(content) => content,
                    Err(_) => {
                        self.error = Some("Failed to read file".to_string());
                        return;
                    }
                };

                // Parse as UTF-8 string
                let file_str = match std::str::from_utf8(&file_content) {
                    Ok(s) => s,
                    Err(_) => {
                        self.error = Some("File is not valid UTF-8".to_string());
                        return;
                    }
                };

                // Parse JSON
                let grids: Vec<ArrayGrid> = match serde_json::from_str(file_str) {
                    Ok(grids) => grids,
                    Err(_) => {
                        self.error = Some("The file is not valid JSON".to_string());
                        return;
                    }
                };

                let mut solved_grids: Vec<ArrayGrid> = Vec::with_capacity(grids.len()); // A vec that we will put the solved grids into

                let start_solving = Instant::now(); // Get the current time

                let solves: Vec<_> = grids
                    .iter()
                    .map(|grid| {
                        let grid = grid.clone();

                        tokio::spawn(async move {
                            handle_solving(grid).await;
                            println!("Just solved a grid");
                        })
                    })
                    .collect();

                // for grid_to_solve in grids.iter() {
                //     // For each grid we have to solve
                //     // We make a new grid object, this object will be easier to manipulate than if we used the 2D array directly
                //     let mut grid = Grid::new();
                //     // We call the function fill_grid from ./sudoku/fill.rs with the grid object, and the grid_to_solve that the JS gave us
                //     grid = fill_grid(grid, grid_to_solve.0);
                //
                //     // We call the solve_grid function, it returns two elements, the solved grid if it can be solved, and success (true = the grid has been solved and is correct, false = the grid cannot be solved and isn't correct)
                //     let (grid, success) = solve_grid(grid);
                //
                //     // If success, we make result the grid object that we turned back into an array, else, we return an empty grid, the JS will then know that the grid isn't correct
                //     let result = if success {
                //         grid.to_array()
                //     } else {
                //         [[0; 9]; 9]
                //     };
                //
                //     // Add the grid we just solved to our array of solved grids
                //     solved_grids.push(ArrayGrid(result));
                //
                //     // Update the progress bar. For now, there is no async support, so the
                //     // rendering will be blocked during solving and the progress bar will not be
                //     // updated
                //     self.json_progress = (grids.iter().count() as f32
                //         / solved_grids.iter().count() as f32)
                //         * 100 as f32;
                //
                //     println!(
                //         "Just solved grid {} out of {}",
                //         &solved_grids.iter().count(),
                //         &grids.iter().count()
                //     );
                // }

                // let solving_duration = start_solving.elapsed().as_millis(); // Calculate the total time we spent solving. We do not use as_secs here as it only return full integers
                // let total_duration = start_total.elapsed().as_millis(); // Calculate the total time with parsing
                //
                // let avg_duration = solving_duration / solved_grids.len() as u128; // Calculate the average time it took per grid
                //
                // self.json_result = Some(format!(
                //     "
                //     Number of grids : {}.
                //     Time spent in total (with json parsing ect...) : {}s.
                //     Time spent actually solving the grids : {}s.
                //     Average time per grid : {}ms.
                // ",
                //     &solved_grids.iter().count(),
                //     solving_duration / 1000,
                //     total_duration / 1000,
                //     avg_duration
                // ));
                //
                // // Create json object of the output
                // let output = serde_json::json!(solved_grids);
                //
                // // Create dialog for storing the output file
                // let file_path = FileDialog::new()
                //     .add_filter("JSON", &["json"])
                //     .set_file_name("solved_grids.json")
                //     .save_file();
                //
                // if let Some(path) = file_path {
                //     match serde_json::to_string_pretty(&output) {
                //         Ok(json) => {
                //             if let Err(e) = std::fs::write(&path, json) {
                //                 self.error = Some(format!("Failed to save: {}", e));
                //             }
                //         }
                //         Err(e) => {
                //             self.error = Some(format!("Failed to serialize: {}", e));
                //         }
                //     }
                // }
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

        let json_progress_bar: ProgressBar = progress_bar(0.0..=100.0, self.json_progress).into();

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
            json_progress_bar,
            json_results_display
        ])
        .into()
    }
}
