// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod algorithms;

#[tauri::command]
fn run_pathfinding(
    colors: Vec<&str>,
    costs: Vec<usize>,
    grid_size: &str,
    algorithm: &str,
) -> Vec<(usize, usize)> {
    let grid = algorithms::Grid::new(
        colors,
        costs,
        grid_size.parse().expect("grid_size must be a number"),
    );

    let path = match algorithm {
        "dijkstra" => algorithms::dijkstra::dijkstra(&grid),
        "a-star" => algorithms::a_star::a_star(&grid),
        "bfs" => algorithms::bfs::bfs(&grid),
        "dfs" => algorithms::dfs::dfs(&grid),
        _ => unreachable!("Unnown algorithm {algorithm}"),
    };

    match path {
        Some(path) => path.iter().map(|&(r, c)| (r, c)).collect(),
        None => vec![],
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_pathfinding])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
