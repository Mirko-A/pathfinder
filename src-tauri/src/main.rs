// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::BinaryHeap;

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

struct Grid {
    cells: Vec<Vec<Cell>>,
    size: usize,
}

impl Grid {
    fn new(colors: Vec<&str>, size: usize) -> Self {
        let cells: Vec<Vec<Cell>> = colors
            .chunks(size)
            .map(|row| row.iter().map(|&c| Cell::from_color(c)).collect())
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

fn dijkstra(grid: &Grid) -> Option<Vec<Pos>> {
    let start = grid.start();
    let end = grid.end();

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let mut dist = vec![vec![usize::MAX; grid.size]; grid.size];
    let mut prev: Vec<Vec<Option<Pos>>> = vec![vec![None; grid.size]; grid.size];

    dist[start.0][start.1] = 0;
    heap.push(Node {
        pos: start,
        cost: 0,
    });

    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    while let Some(Node { pos, cost }) = heap.pop() {
        if pos == end {
            let mut path = vec![];
            let mut current = pos;

            while let Some(prev_pos) = prev[current.0][current.1] {
                path.push(current);
                current = prev_pos;
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        if cost > dist[pos.0][pos.1] {
            continue;
        }

        'dir_loop: for (dx, dy) in directions.iter() {
            let next_row = (pos.0 as isize + dx) as usize;
            let next_col = (pos.1 as isize + dy) as usize;

            if let Some(cell) = grid.get((next_row, next_col)) {
                if cell == Cell::Blocked {
                    continue 'dir_loop;
                }

                let next_cost = cost + 1;
                if next_cost < dist[next_row][next_col] {
                    dist[next_row][next_col] = next_cost;
                    heap.push(Node {
                        pos: (next_row, next_col),
                        cost: next_cost,
                    });
                    prev[next_row][next_col] = Some(pos);
                }
            }
        }
    }

    None
}

#[tauri::command]
fn debug(colors: Vec<&str>, grid_size: &str) -> Vec<(usize, usize)> {
    let grid = Grid::new(
        colors,
        grid_size.parse().expect("grid_size must be a number"),
    );
    let path = dijkstra(&grid);

    match path {
        Some(path) => path.iter().map(|&(r, c)| (r, c)).collect(),
        None => vec![],
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![debug])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
