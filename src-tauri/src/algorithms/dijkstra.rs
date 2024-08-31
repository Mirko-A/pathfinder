use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::algorithms::{CellType, Grid, Node, Pos};

pub fn dijkstra(grid: &Grid) -> Option<Vec<Pos>> {
    let start = grid.start();
    let end = grid.end();

    let mut heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    let mut dist = vec![vec![usize::MAX; grid.size]; grid.size];
    let mut prev: Vec<Vec<Option<Pos>>> = vec![vec![None; grid.size]; grid.size];

    dist[start.0][start.1] = 0;
    heap.push(Reverse(Node {
        pos: start,
        cost: 0,
    }));

    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    while let Some(Reverse(Node { pos, cost })) = heap.pop() {
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
                if cell.typ == CellType::Blocked {
                    continue 'dir_loop;
                }

                let next_cost = cost + cell.cost;
                if next_cost < dist[next_row][next_col] {
                    dist[next_row][next_col] = next_cost;
                    heap.push(Reverse(Node {
                        pos: (next_row, next_col),
                        cost: next_cost,
                    }));
                    prev[next_row][next_col] = Some(pos);
                }
            }
        }
    }

    None
}
