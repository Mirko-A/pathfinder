pub mod a_star;
pub mod bfs;
pub mod dfs;
pub mod dijkstra;

#[derive(Clone, Copy, Eq, PartialEq)]
enum CellType {
    Start,
    End,
    Empty,
    Blocked,
}

#[derive(Clone, Copy)]
struct Cell {
    typ: CellType,
    cost: usize,
}

impl Cell {
    fn new(color: &str, cost: usize) -> Self {
        let typ = match color {
            "green" => CellType::Start,
            "red" => CellType::End,
            "black" => CellType::Blocked,
            "white" => CellType::Empty,
            _ => unreachable!("unrecognized color {}", color),
        };

        Self { typ, cost }
    }
}

// (row, col)
type Pos = (usize, usize);

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    size: usize,
}

impl Grid {
    pub fn new(colors: Vec<&str>, costs: Vec<usize>, size: usize) -> Self {
        let cells: Vec<_> = colors.iter().zip(costs).collect();
        let cells: Vec<Vec<Cell>> = cells
            .chunks(size)
            .map(|row| {
                row.iter()
                    .map(|&(color, cost)| Cell::new(color, cost))
                    .collect()
            })
            .collect();

        Self { cells, size }
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
                if cell.typ == CellType::Start {
                    return (r, c);
                }
            }
        }
        unreachable!("no start cell found");
    }

    fn end(&self) -> Pos {
        for (r, row) in self.cells.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if cell.typ == CellType::End {
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
