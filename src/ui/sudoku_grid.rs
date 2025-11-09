use crate::resolver::{Grid, fill_grid, solve_grid};
use iced::{
    Element, Pixels,
    widget::{Column, Container, Row, Text, button, column, container, row, text, text_input},
};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct SudokuGrid {
    value: Grid,
    error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArrayGrid([[u8; 9]; 9]);

#[derive(Debug, Clone)]
pub enum SudokuMessage {
    InputChanged { x: usize, y: usize, value: String },
    Solve,
    Reset,
    LoadJson,
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
                let file_path = FileDialog::new()
                    .add_filter("text", &["json"])
                    .set_directory("/")
                    .pick_file();

                println!("Got the file");

                // let start_total = Date::now(); // Get the current time (for calculating how much time it took)

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

                // let start_solving = Date::now(); // Get the current time

                for grid_to_solve in grids.iter() {
                    // For each grid we have to solve
                    // We make a new grid object, this object will be easier to manipulate than if we used the 2D array directly
                    let mut grid = Grid::new();
                    // We call the function fill_grid from ./sudoku/fill.rs with the grid object, and the grid_to_solve that the JS gave us
                    grid = fill_grid(grid, grid_to_solve.0);

                    // We call the solve_grid function, it returns two elements, the solved grid if it can be solved, and success (true = the grid has been solved and is correct, false = the grid cannot be solved and isn't correct)
                    let (grid, success) = solve_grid(grid);

                    // If success, we make result the grid object that we turned back into an array, else, we return an empty grid, the JS will then know that the grid isn't correct
                    let result = if success {
                        grid.to_array()
                    } else {
                        [[0; 9]; 9]
                    };

                    // Add the grid we just solved to our array of solved grids
                    solved_grids.push(ArrayGrid(result));

                    println!(
                        "Just solved grid {} out of {}",
                        &solved_grids.iter().count(),
                        &grids.iter().count()
                    );
                }

                // let solving_duration = Date::now() - start_solving; // Calculate the total time we spent solving
                // let total_duration = Date::now() - start_total; // Calculate the total time with parsing

                // let avg_duration = solving_duration / solved_grids.len() as f64; // Calculate the average time it took per grid

                println!("Solved grids : {:?}", solved_grids);
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

        column![error_display, grid, buttons, load_json_button].into()
    }
}
