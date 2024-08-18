pub mod a_star;
pub mod dijkstra;
pub mod grid;

#[allow(dead_code)]

pub trait PathfindingAlgorithm {
    fn find_shortest_path(start: grid::Point, end: grid::Point, grid: &grid::Grid);
}
