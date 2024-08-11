use eframe::egui;

fn main() -> eframe::Result<()> {
    let app = Pathfinder::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Pathfinder", native_options, Box::new(|cc| Box::new(app)))
}

#[derive(Debug, Clone, Copy, Default)]
struct Square {
    state: u32,
}

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    square_size: usize,
    squares: Vec<Square>,
}

impl Grid {
    fn new(rows: usize, cols: usize, square_size: usize) -> Self {
        let mut squares = Vec::new();

        for i in 0..(rows * cols) {
            let state = if i % 2 == 0 { 1 } else { 0 };
            squares.push(Square { state });
        }

        Self {
            rows,
            cols,
            square_size,
            squares,
        }
    }
}

const GRID_ROWS: usize = 8;
const GRID_COLS: usize = 8;
const SQUARE_SIZE: usize = 50;

impl Default for Grid {
    fn default() -> Self {
        Self::new(GRID_ROWS, GRID_COLS, SQUARE_SIZE)
    }
}

#[derive(Default)]
struct Pathfinder {
    grid: Grid,
}

impl eframe::App for Pathfinder {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();

            for y in 0..self.grid.rows {
                for x in 0..self.grid.cols {
                    let tl = egui::pos2(
                        (x * self.grid.square_size) as f32,
                        (y * self.grid.square_size) as f32,
                    );
                    let size =
                        egui::vec2(self.grid.square_size as f32, self.grid.square_size as f32);
                    let r = egui::Rect::from_min_size(tl, size);
                    let c = if (x + y) % 2 == 0 {
                        egui::Color32::KHAKI
                    } else {
                        egui::Color32::RED
                    };

                    painter.rect_filled(r, 0.0, c);
                }
            }
        });
    }
}
