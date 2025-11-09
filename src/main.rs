// use resolver::{Grid, fill_grid, solve_grid};
use ui::SudokuGrid;

mod resolver;
mod ui;

fn main() -> iced::Result {
    iced::application("Sudoku native", SudokuGrid::update, SudokuGrid::view)
        .theme(|_| iced::Theme::Dark)
        .run()
}
