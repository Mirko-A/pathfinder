// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod algorithms;

#[tauri::command]
fn run_pathfinding(colors: Vec<&str>, grid_size: &str) -> Vec<(usize, usize)> {
    let grid = algorithms::Grid::new(
        colors,
        grid_size.parse().expect("grid_size must be a number"),
    );

    let path = algorithms::dijkstra::dijkstra(&grid);

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
