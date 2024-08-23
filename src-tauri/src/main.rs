// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    collections::{HashMap, HashSet},
    isize,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Kind {
    Empty,
    Block,
    Start,
    End,
}

#[derive(Debug)]
struct Block {
    point: Point,
    kind: Kind,
}

#[derive(Debug)]
struct Grid {
    blocks: Vec<Block>,
    size: usize,
    start: Point,
    end: Point,
}

struct Dir(i8, i8);

impl Dir {
    fn left() -> Self {
        Dir(-1, 0)
    }
    fn right() -> Self {
        Dir(1, 0)
    }
    fn up() -> Self {
        Dir(0, -1)
    }
    fn down() -> Self {
        Dir(0, 1)
    }
}

fn find_neighbors(grid: &Grid, me: &Point) -> [Option<Point>; 4] {
    let left = if me.x > 0 {
        let p = Point {
            x: me.x - 1,
            y: me.y,
        };

        if grid.blocks[p.x * grid.size + p.y].kind == Kind::Block {
            None
        } else {
            Some(p)
        }
    } else {
        None
    };

    let right = if me.x < grid.size - 1 {
        let p = Point {
            x: me.x + 1,
            y: me.y,
        };

        if grid.blocks[p.x * grid.size + p.y].kind == Kind::Block {
            None
        } else {
            Some(p)
        }
    } else {
        None
    };

    let up = if me.y > 0 {
        let p = Point {
            x: me.x,
            y: me.y - 1,
        };

        if grid.blocks[p.x * grid.size + p.y].kind == Kind::Block {
            None
        } else {
            Some(p)
        }
    } else {
        None
    };

    let down = if me.y < grid.size - 1 {
        let p = Point {
            x: me.x,
            y: me.y + 1,
        };

        if grid.blocks[p.x * grid.size + p.y].kind == Kind::Block {
            None
        } else {
            Some(p)
        }
    } else {
        None
    };

    [left, right, up, down]
}

fn dijkstra(grid: &Grid) {
    let mut dist: HashMap<Point, u32> = HashMap::new();
    let mut prev: HashMap<Point, Option<Point>> = HashMap::new();
    let mut q: HashSet<Point> = HashSet::new();

    for b in &grid.blocks {
        dist.insert(b.point, u32::MAX);
        prev.insert(b.point, None);
        q.insert(b.point);
    }

    dist.insert(grid.start, 0);

    let mut u = grid.start;
    assert!(q.remove(&grid.start), "node must be in the set");

    while !q.is_empty() {
        let nbors = find_neighbors(grid, &u);

        for n in nbors.iter().flatten() {
            // TODO:
            // + 1 should be replaced with the distance between u and n
            let alt = dist[&u] + 1;
            if alt < dist[n] {
                dist.insert(*n, alt);
                prev.insert(*n, Some(u));
            }
        }

        // u = point with minimum distance from current u
        // remove u from q
    }
}

#[tauri::command]
fn debug(squares: Vec<&str>, grid_size: &str) {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    let mut blocks: Vec<Block> = Vec::new();
    let grid_size: usize = grid_size.parse().expect("grid size is a number");

    for i in 0..grid_size {
        for j in 0..grid_size {
            let kind = match squares[i * grid_size + j] {
                "white" => Kind::Empty,
                "black" => Kind::Block,
                "green" => Kind::Start,
                "red" => Kind::End,
                _ => panic!("invalid square type"),
            };
            if kind == Kind::Start {
                start = Some(Point { x: i, y: j });
            } else if kind == Kind::End {
                end = Some(Point { x: i, y: j });
            }
            blocks.push(Block {
                point: Point { x: i, y: j },
                kind,
            });
        }
    }

    assert!(
        start.is_some() && end.is_some(),
        "front-end must ensure that START and END node are selected"
    );

    let grid = Grid {
        blocks,
        size: grid_size,
        start: start.unwrap(),
        end: end.unwrap(),
    };

    dijkstra(&grid);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![debug])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
