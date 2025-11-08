use crate::resolver::grid; // Import the Grid struct from ./grid.rs

// This function fills the grid from a 2D array, it gets the grid that is a Grid object, and the 2D array, and returns a Grid object that is the grid filled with JS's values
pub fn fill_grid(mut grid: grid::Grid, grid_to_fill: [[u8; 9]; 9]) -> grid::Grid {
    // For each row and col
    for row in 0..9 {
        for col in 0..9 {
            let value = grid_to_fill[row][col]; // We get the value from the grid sent by the JS
            grid.set(row, col, value); // We set the cell to the value from the grid sent by the JS
        }
    }

    grid // We return the filled grid
}
