use crate::resolver::{Grid, solve_grid};
use iced::{
    Element, Pixels,
    widget::{Button, Column, Text, button, column, row, text, text_input},
};

#[derive(Default)]
pub struct SudokuGrid {
    value: Grid,
    error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SudokuMessage {
    InputChanged { x: usize, y: usize, value: String },
    Solve,
}

impl SudokuGrid {
    pub fn update(&mut self, message: SudokuMessage) {
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

        let solve_button: Button<SudokuMessage> =
            button("Solve the grid").on_press(SudokuMessage::Solve);

        let error_display: Text = text({
            match &self.error {
                Some(err) => err.as_str(),
                None => "",
            }
        })
        .style(text::danger);

        column![error_display, grid, solve_button].into()
    }
}
