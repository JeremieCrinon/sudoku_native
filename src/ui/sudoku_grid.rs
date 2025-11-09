use crate::resolver::Grid;
use iced::{
    widget::{
        column,
        row,
        text_input,
        Column
    },
    Element
};

#[derive(Default)]
pub struct SudokuGrid {
    value: Grid,
}

#[derive(Debug, Clone)]
pub enum SudokuMessage {
    InputChanged { x: usize, y: usize, value: String },
}

impl SudokuGrid {
    pub fn update(&mut self, message: SudokuMessage) {
        match message {
            SudokuMessage::InputChanged { x, y, value } => {
                // Parse the input to a number an make it 0 if it's not valid
                let value_trimmed = value.trim();

                let value_parsed = if !value_trimmed.is_empty() && value_trimmed.len() == 1 {
                    value_trimmed.chars().next().and_then(|c| c.to_digit(10).map(|d| d as u8))
                } else {
                    None
                };

                if let Some(num) = value_parsed {
                    self.value.set(x, y, num);
                } else {
                    self.value.set(x, y, 0);
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
                        move |v| SudokuMessage::InputChanged {
                            x, y, value: v,
                        }
                    })
                    .into()
            });

            row(cells).into()
        });

        column(rows).into()

    }

}
