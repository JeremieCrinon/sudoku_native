use resolver::{
    fill_grid, 
    solve_grid, 
    Grid
};
use ui::SudokuGrid;

mod resolver;
mod ui;


fn main() -> iced::Result {
    iced::application("Sudoku native", SudokuGrid::update, SudokuGrid::view)
        .run()
}
