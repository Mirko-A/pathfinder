use eframe::egui;

fn main() -> eframe::Result<()> {
    let app = Pathfinder::default();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(app.win_props.width, app.win_props.height)),
        ..Default::default()
    };
    eframe::run_native(
        app.win_props.title.clone().as_str(),
        native_options,
        Box::new(|cc| Box::new(app)),
    )
}

#[derive(Clone, Debug, Default)]
enum SquareState {
    #[default]
    Empty,
    Blocked,
    Start,
    End,
}

const EMPTY_SQUARE_COLOR: egui::Color32 = egui::Color32::WHITE;
const BLOCKED_SQUARE_COLOR: egui::Color32 = egui::Color32::BLACK;
const START_SQUARE_COLOR: egui::Color32 = egui::Color32::GREEN;
const END_SQUARE_COLOR: egui::Color32 = egui::Color32::RED;
const SELECTED_SQUARE_COLOR: egui::Color32 = egui::Color32::DARK_BLUE;
const GRID_LINE_COLOR: egui::Color32 = egui::Color32::BLACK;

impl From<&SquareState> for egui::Color32 {
    fn from(state: &SquareState) -> egui::Color32 {
        match state {
            SquareState::Empty => EMPTY_SQUARE_COLOR,
            SquareState::Blocked => BLOCKED_SQUARE_COLOR,
            SquareState::Start => START_SQUARE_COLOR,
            SquareState::End => END_SQUARE_COLOR,
        }
    }
}

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    square_size: usize,
    squares: Vec<SquareState>,
}

impl Grid {
    fn new(rows: usize, cols: usize, square_size: usize) -> Self {
        let squares = vec![SquareState::default(); rows * cols];

        Self {
            rows,
            cols,
            square_size,
            squares,
        }
    }
}

const GRID_ROWS: usize = 10;
const GRID_COLS: usize = 10;
const SQUARE_SIZE: usize = 70;

impl Grid {
    fn default() -> Self {
        Self::new(GRID_ROWS, GRID_COLS, SQUARE_SIZE)
    }
}

struct WindowProps {
    title: String,
    width: f32,
    height: f32,
}

const APP_TITLE: &str = "Pathfinder";
const APP_WIDTH: f32 = (GRID_COLS * SQUARE_SIZE) as f32;
const APP_HEIGHT: f32 = (GRID_ROWS * SQUARE_SIZE) as f32;

impl Default for WindowProps {
    fn default() -> Self {
        Self {
            title: APP_TITLE.to_string(),
            width: APP_WIDTH,
            height: APP_HEIGHT,
        }
    }
}

struct Pathfinder {
    win_props: WindowProps,
    selected_square: (usize, usize),
    grid: Grid,
}

impl Default for Pathfinder {
    fn default() -> Self {
        let grid = Grid::default();

        Self {
            win_props: WindowProps {
                title: APP_TITLE.to_string(),
                width: (grid.cols * grid.square_size) as f32,
                height: (grid.rows * grid.square_size) as f32,
            },
            selected_square: (0, 0),
            grid,
        }
    }
}

const SELECTED_SQUARE_WIDTH: f32 = 3.0;
const GRID_LINE_WIDTH: f32 = 1.0;

impl Pathfinder {
    fn update_state(&mut self, ctx: &egui::Context) {
        self.update_selected_square(ctx);
        self.update_grid(ctx);
    }

    fn update_selected_square(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::ArrowLeft) && self.selected_square.0 != 0 {
                self.selected_square.0 -= 1;
            }
            if i.key_pressed(egui::Key::ArrowRight) && self.selected_square.0 < self.grid.cols - 1 {
                self.selected_square.0 += 1;
            }
            if i.key_pressed(egui::Key::ArrowUp) && self.selected_square.1 != 0 {
                self.selected_square.1 -= 1;
            }
            if i.key_pressed(egui::Key::ArrowDown) && self.selected_square.1 < self.grid.rows - 1 {
                self.selected_square.1 += 1;
            }
        });
    }

    fn update_grid(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Space) {
                let square = &mut self.grid.squares
                    [self.selected_square.1 * self.grid.cols + self.selected_square.0];
                *square = match square {
                    SquareState::Empty => SquareState::Blocked,
                    SquareState::Blocked => SquareState::Start,
                    SquareState::Start => SquareState::End,
                    SquareState::End => SquareState::Empty,
                };
            }
        });
    }

    fn draw(&self, painter: &egui::Painter) {
        self.draw_grid(painter);
        self.draw_selected_square(painter);
    }

    fn draw_grid(&self, painter: &egui::Painter) {
        // Draw squares
        for i in 0..self.grid.rows {
            for j in 0..self.grid.cols {
                let square = &self.grid.squares[i * self.grid.cols + j];
                let color: egui::Color32 = square.into();

                let rect = egui::Rect::from_min_max(
                    egui::pos2(
                        j as f32 * self.grid.square_size as f32,
                        i as f32 * self.grid.square_size as f32,
                    ),
                    egui::pos2(
                        (j + 1) as f32 * self.grid.square_size as f32,
                        (i + 1) as f32 * self.grid.square_size as f32,
                    ),
                );

                painter.rect_filled(rect, 0.0, color);
            }
        }

        // Draw grid lines
        // Horizontal
        for i in 0..self.grid.rows {
            let y = i as f32 * self.grid.square_size as f32;
            let start = egui::pos2(0.0, y);
            let end = egui::pos2(self.grid.cols as f32 * self.grid.square_size as f32, y);
            painter.line_segment(
                [start, end],
                egui::Stroke::new(GRID_LINE_WIDTH, GRID_LINE_COLOR),
            );
        }
        // Vertical
        for i in 0..self.grid.cols {
            let x = i as f32 * self.grid.square_size as f32;
            let start = egui::pos2(x, 0.0);
            let end = egui::pos2(x, self.grid.rows as f32 * self.grid.square_size as f32);
            painter.line_segment(
                [start, end],
                egui::Stroke::new(GRID_LINE_WIDTH, GRID_LINE_COLOR),
            );
        }
    }

    fn draw_selected_square(&self, painter: &egui::Painter) {
        let min = egui::pos2(
            (self.selected_square.0 * self.grid.square_size) as f32,
            (self.selected_square.1 * self.grid.square_size) as f32,
        );
        let size = egui::vec2(self.grid.square_size as f32, self.grid.square_size as f32);
        let select_rect = egui::Rect::from_min_size(min, size);
        painter.rect(
            select_rect,
            0.0,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(SELECTED_SQUARE_WIDTH, SELECTED_SQUARE_COLOR),
        );
    }
}

impl eframe::App for Pathfinder {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_state(ctx);
            self.draw(ui.painter());
        });
    }
}
