pub mod dijkstra;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell {
    Start,
    End,
    Empty,
    Blocked,
}

impl Cell {
    fn from_color(c: &str) -> Self {
        match c {
            "green" => Cell::Start,
            "red" => Cell::End,
            "black" => Cell::Blocked,
            "white" => Cell::Empty,
            _ => unreachable!("unrecognized color {}", c),
        }
    }
}

// (row, col)
type Pos = (usize, usize);

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    costs: Vec<Vec<u8>>,
    size: usize,
}

impl Grid {
    pub fn new(colors: Vec<&str>, costs: Vec<u8>, size: usize) -> Self {
        let cells: Vec<Vec<Cell>> = colors
            .chunks(size)
            .map(|row| row.iter().map(|&c| Cell::from_color(c)).collect())
            .collect();
        let costs = costs.chunks(size).map(|row| row.to_vec()).collect();

        Self { cells, costs, size }
    }

    fn get(&self, pos: Pos) -> Option<Cell> {
        if pos.0 < self.size && pos.1 < self.size {
            Some(self.cells[pos.0][pos.1])
        } else {
            None
        }
    }

    fn start(&self) -> Pos {
        for (r, row) in self.cells.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if *cell == Cell::Start {
                    return (r, c);
                }
            }
        }
        unreachable!("no start cell found");
    }

    fn end(&self) -> Pos {
        for (r, row) in self.cells.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if *cell == Cell::End {
                    return (r, c);
                }
            }
        }
        unreachable!("no end cell found");
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    pos: Pos,
    cost: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}
