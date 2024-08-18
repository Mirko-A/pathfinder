mod egui_app;
mod grid;

fn main() -> eframe::Result<()> {
    let app = egui_app::Pathfinder::default();
    egui_app::Pathfinder::run(app)
}
