use rand;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Square {
    #[default]
    Empty,
    Blocked,
    Start,
    End,
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Square::Empty => write!(f, "Empty"),
            Square::Blocked => write!(f, "Blocked"),
            Square::Start => write!(f, "Start"),
            Square::End => write!(f, "End"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub squares: Vec<Square>,
    pub start: Option<Point>,
    pub end: Option<Point>,
}

pub const DEFAULT_GRID_ROWS: usize = 25;
pub const DEFAULT_GRID_COLS: usize = 25;

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let squares = vec![Square::default(); rows * cols];

        Self {
            rows,
            cols,
            squares,
            start: None,
            end: None,
        }
    }

    pub fn default() -> Self {
        Self::new(DEFAULT_GRID_ROWS, DEFAULT_GRID_COLS)
    }

    pub fn with_random_blockage(rows: usize, cols: usize) -> Self {
        let mut squares = vec![Square::default(); rows * cols];

        for square in squares.iter_mut() {
            if rand::random::<bool>() {
                *square = Square::Blocked;
            }
        }

        Self {
            rows,
            cols,
            squares,
            start: None,
            end: None,
        }
    }
}
