mod resolver;
mod ui;

use ui::SudokuGrid;

fn main() -> iced::Result {
    iced::application("Sudoku Native", SudokuGrid::update, SudokuGrid::view)
        .run_with(|| SudokuGrid::new())
}
