#[derive(Debug, Clone, Default)]
pub struct Grid { // The structure of the grid
    cells: [[u8; 9]; 9], // We want an array containing 9 arrays containing 9 u8
}

// We add some methodes to Grid
impl Grid {
    // Create a new empty grid
    pub fn new() -> Self {
        Grid { cells: [[0; 9]; 9] }
    }

    // Set a value for a cell of the grid
    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.cells[row][col] = value;
    }

    // Get a value for a cell of the grid
    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.cells[row][col]
    }

    // Turn the grid back to a simple 2D array
    pub fn to_array(&self) -> [[u8; 9]; 9] {
        self.cells
    }
}
