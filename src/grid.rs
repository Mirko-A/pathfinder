#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum SquareState {
    #[default]
    Empty,
    Blocked,
    Start,
    End,
}

#[derive(Debug)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub square_size: usize,
    pub squares: Vec<SquareState>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize, square_size: usize) -> Self {
        let squares = vec![SquareState::default(); rows * cols];

        Self {
            rows,
            cols,
            square_size,
            squares,
        }
    }
}

pub const GRID_ROWS: usize = 25;
pub const GRID_COLS: usize = 25;
pub const SQUARE_SIZE: usize = 40;

impl Grid {
    pub fn default() -> Self {
        Self::new(GRID_ROWS, GRID_COLS, SQUARE_SIZE)
    }
}
