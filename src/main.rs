use ui::SudokuGrid;

mod resolver;
mod ui;

#[tokio::main]
async fn main() -> iced::Result {
    iced::application("Sudoku native", SudokuGrid::update, SudokuGrid::view)
        .theme(|_| iced::Theme::Dark)
        .run()
}
