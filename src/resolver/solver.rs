use crate::resolver::grid; // The grid struct from ./grid.rs
use std::collections::HashSet;

// This function gets a 1D array, and returns a boolean to tell wether or not there is duplicates
fn check_array_dupes(array: &[u8]) -> bool {
    let mut seen = HashSet::new();
    for &value in array.iter() {
        if value != 0 && !seen.insert(value) {
            return false; // Found a duplicate
        }
    }

    true // No duplicates
}

// Gets a row and tells if it is valid (does not have twice the same number)
fn is_row_valid(grid: &grid::Grid, row_num: usize) -> bool {
    // We make a row variable that is a 1D array containing 9 u8
    let mut row: [u8; 9] = [0; 9];
    // For each cell (column) of the row
    for col in 0..9 {
        // We set the value of the row array to the value in the grid
        row[col] = grid.get(row_num, col);
    }

    // We call the function check_array_dupes to check if there is duplicates in here
    check_array_dupes(&row)
}

// Same as is_row_valid
fn is_col_valid(grid: &grid::Grid, col_num: usize) -> bool {
    let mut col: [u8; 9] = [0; 9];
    for row in 0..9 {
        col[row] = grid.get(row, col_num);
    }

    check_array_dupes(&col)
}

// Gets a square, and tells if it is valid (does not have twice the same number)
fn is_square_valid(grid: &grid::Grid, row_off: usize, col_off: usize) -> bool {
    // A 1D array representing the square
    let mut square: [u8; 9] = [0; 9];
    // The index to set a value in the square idex
    let mut idx: usize = 0;

    // For each of the 3 row and cols of the square
    for i in 0..3 {
        for j in 0..3 {
            // We add to the square 1D array what we got from the grid
            square[idx] = grid.get(row_off + i, col_off + j);
            idx += 1;
        }
    }

    // We check for dupes in the ID array containing all values from the 2D square
    check_array_dupes(&square)
}

// This functions gets the grid and tells if it is valid (does not have any duplicates in any row, col and square)
fn check_grid(grid: &grid::Grid) -> bool {
    // For each of the rows and cols
    for i in 0..9 {
        // If the row or column isn't valid (has twice or more the same number)...
        if !is_row_valid(grid, i) || !is_col_valid(grid, i) {
            // We return false, no need to continue, the grid already isn't valid
            return false;
        }
    }

    // For each of the squares, we start to the square from 0;0, and 0;3, 0;6, 3;0 ...
    for i in (0..7).step_by(3) {
        for j in (0..7).step_by(3) {
            // We verify if the square is valid
            if !is_square_valid(grid, i, j) {
                // If it isn't valid, we return false, no need to continue, the grid already isn't valid
                return false;
            }
        }
    }

    // If we got to here, it means the grid is valid
    true
}

// Gets the first empty cell
fn get_first_element(grid: &grid::Grid) -> (usize, usize) {
    // We go trough all of the cells
    for i in 0..9 {
        for j in 0..9 {
            // If the cell is empty
            if grid.get(i, j) == 0 {
                // We return the cordinates of the cell, and, because we return, the rest of the for won't be executed, it won't be really optimised else
                return (i, j);
            }
        }
    }

    // If we did not find any empty cell, it means the grid is filled, we return 0, 0 to tell that
    (0, 0)
}

// This is the function to solve the grid, it gets a Grid object, and return a Grid object plus a boolean, true if the grid is valid, false if it ain't
pub fn solve_grid(mut grid: grid::Grid) -> (grid::Grid, bool) {
    // We get the coordinates of the first empty element in the grid
    let (row, col) = get_first_element(&grid);

    // We go from 1 to 9, every number there can be in a grid
    for i in 1..10 {
        // We set the number we are currently on in the cell
        grid.set(row, col, i);

        // We get wether or not the grid is valid
        let grid_correct = check_grid(&grid);

        // If it is...
        if grid_correct {
            // We call the function to get the first element again, but this time, it is to know if the grid is filled or not (the function does both)
            let (r_row, r_col) = get_first_element(&grid);

            // If it is 0, 0...
            if r_row == 0 && r_col == 0 {
                // We return the grid, and that it is valid
                return (grid, true);
            }
            // If you wonder, no, if the JS sends a grid with the first cell empty, wed won't have a problem caused by the first empty cell being a the cordinates 0; 0, which would interfer with the grid being full, because we always edit the first empty cell before verifing the grid is filled, so when we call the function again to verify the grid is filled, the 0; 0 cell will never be empty

            // We call the solve_grid function again (recursivity) with a copy of the grid
            let (solved_grid, result) = solve_grid(grid.clone());

            // If it returned true, it means that it filled the grid, so we return the grid it returned to us, with true, and the function that called it return the grid we are returning too, ect... until we get to the call from lib.rs
            if result {
                return (solved_grid, true);
            }

            // Else, it means it returned false, so, even if the grid is currently valid, it isn't the right number cause later, it has no number that can be valid
        }
    }

    // If we got here, it means we tried every number, and none were valid, so we returne false, and the parent function will try another number, and if it has none left, it will return false to the parent ect...
    (grid, false)
}
