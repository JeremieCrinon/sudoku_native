use crate::resolver::Grid;
use iced::{
    widget::{
        column,
        row,
        text,
        Column
    },
    Element
};

#[derive(Default)]
pub struct SudokuGrid {
    value: Grid,
}

#[derive(Debug, Clone, Copy)]
pub enum SudokuMessage {

}

impl SudokuGrid {
    pub fn update(&mut self, _message: SudokuMessage) {

    }

    pub fn view(&self) -> Element<'_, SudokuMessage> {

        let rows = (0..9).map(|y| {
            let cells = (0..9).map(|x| {
                text(self.value.get(x, y)).into()
            });

            row(cells).into()
        });

        column(rows).into()

    }

}
