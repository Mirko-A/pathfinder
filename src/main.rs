mod egui_app;
mod grid;

fn main() -> eframe::Result<()> {
    //    let app = egui_app::Pathfinder::default();
    let app = egui_app::Pathfinder::new(20, 20, 50);
    egui_app::Pathfinder::run(app)
}
