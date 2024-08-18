mod algorithms;
mod egui_app;

fn main() -> eframe::Result<()> {
    // let app = egui_app::Pathfinder::default();
    let app = egui_app::Pathfinder::new(30, 30, 33, true);
    egui_app::Pathfinder::run(app)
}
