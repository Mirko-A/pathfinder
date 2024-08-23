// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::{HashMap, HashSet};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) {
    println!("Hello, {}! You've been greeted from Rust!", name);
}

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
    blocks: Vec<Vec<Block>>,
    size: usize,
    start: Point,
    end: Point,
}
fn find_min_dist(q: &HashSet<Point>, u: &Point) -> Point {
    let mut min_dist = usize::MAX;
    let mut min_point = &Point { x: 0, y: 0 };

    for point in q {
        let dist = ((u.x as isize - point.x as isize).abs()
            + (u.y as isize - point.y as isize).abs()) as usize;
        if dist < min_dist {
            min_dist = dist;
            min_point = point;
        }
    }

    *min_point
}

fn find_shortest_path(grid: &Grid) {
    let mut dist: HashMap<Point, usize> = grid
        .blocks
        .iter()
        .flatten()
        .map(|b| (b.point, usize::MAX))
        .collect();
    let prev: HashMap<Point, Option<Point>> = grid
        .blocks
        .iter()
        .flatten()
        .map(|b| (b.point, None))
        .collect();

    let mut q: HashSet<Point> = grid.blocks.iter().flatten().map(|b| b.point).collect();

    let mut u = grid.start;
    let mut alt: usize;

    while !q.is_empty() {
        u = find_min_dist(&q, &u);
        assert!(q.remove(&u), "`u` must be present in `q`");

        // TODO:
        // Finish dijkstra's algorithm
    }

    println!("Finding shortest path");
    println!("{:?}", grid);
}

#[tauri::command]
fn debug(squares: Vec<&str>, grid_size: &str) {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    let mut grid: Vec<Vec<Block>> = Vec::new();
    let grid_size: usize = grid_size.parse().expect("grid size is a number");

    for i in 0..grid_size {
        grid.push(Vec::new());

        for j in 0..grid_size {
            let square = match squares[i * grid_size + j] {
                "white" => Kind::Empty,
                "black" => Kind::Block,
                "green" => Kind::Start,
                "red" => Kind::End,
                _ => panic!("invalid square type"),
            };
            if square == Kind::Start {
                start = Some(Point { x: i, y: j });
            } else if square == Kind::End {
                end = Some(Point { x: i, y: j });
            }
            grid[i].push(Block {
                point: Point { x: i, y: j },
                kind: square,
            });
        }
    }

    assert!(
        start.is_some() && end.is_some(),
        "front-end must ensure that START and END node are selected"
    );

    let grid = Grid {
        blocks: grid,
        size: grid_size,
        start: start.unwrap(),
        end: end.unwrap(),
    };

    find_shortest_path(&grid);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, debug])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
