use eframe::egui;
use std::fmt::{self, Display, Formatter};

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

const GRID_ROWS: usize = 25;
const GRID_COLS: usize = 25;
const SQUARE_SIZE: usize = 40;

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

#[derive(PartialEq, Eq)]
enum GridmakerMode {
    AddEmpty,
    AddBlocked,
    AddStart,
    AddEnd,
}

impl Display for GridmakerMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            GridmakerMode::AddEmpty => write!(f, "Add Empty"),
            GridmakerMode::AddBlocked => write!(f, "Add Blocked"),
            GridmakerMode::AddStart => write!(f, "Add Start"),
            GridmakerMode::AddEnd => write!(f, "Add End"),
        }
    }
}

struct Pathfinder {
    win_props: WindowProps,
    grid_area: egui::Rect,
    status_area: egui::Rect,
    selected_square: (usize, usize),
    gm_mode: GridmakerMode,
    start_square: Option<(usize, usize)>,
    end_square: Option<(usize, usize)>,
    grid: Grid,
}

const STATUS_AREA_HEIGHT: f32 = 75.0;

impl Default for Pathfinder {
    fn default() -> Self {
        let grid = Grid::default();

        let grid_width = grid.cols as f32 * grid.square_size as f32;
        let grid_height = grid.rows as f32 * grid.square_size as f32;

        let grid_area =
            egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(grid_width, grid_height));

        let status_area_height = STATUS_AREA_HEIGHT;
        let status_area = egui::Rect::from_min_size(
            egui::pos2(0.0, grid_height),
            egui::vec2(grid_width, status_area_height),
        );

        Self {
            win_props: WindowProps {
                title: APP_TITLE.to_string(),
                width: grid_area.width(),
                height: grid_area.height() + status_area.height(),
            },
            grid_area,
            status_area,
            selected_square: (0, 0),
            gm_mode: GridmakerMode::AddEmpty,
            start_square: None,
            end_square: None,
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
        self.update_gm_mode(ctx);
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
        let idx = self.selected_square.1 * self.grid.cols + self.selected_square.0;
        match self.gm_mode {
            GridmakerMode::AddEmpty | GridmakerMode::AddBlocked => {
                if let Some(pos) = self.start_square {
                    if pos == self.selected_square {
                        self.start_square = None;
                    }
                }
                if let Some(pos) = self.end_square {
                    if pos == self.selected_square {
                        self.end_square = None;
                    }
                }

                if self.gm_mode == GridmakerMode::AddEmpty {
                    self.grid.squares[idx] = SquareState::Empty;
                } else {
                    self.grid.squares[idx] = SquareState::Blocked;
                }
            }
            GridmakerMode::AddStart => {
                if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if let Some(pos) = self.start_square {
                        self.grid.squares[pos.1 * self.grid.cols + pos.0] = SquareState::Empty;
                    }
                    self.start_square = Some(self.selected_square);
                    self.grid.squares[idx] = SquareState::Start;
                }
            }
            GridmakerMode::AddEnd => {
                if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if let Some(pos) = self.end_square {
                        self.grid.squares[pos.1 * self.grid.cols + pos.0] = SquareState::Empty;
                    }
                    self.end_square = Some(self.selected_square);
                    self.grid.squares[idx] = SquareState::End;
                }
            }
        }
    }

    fn update_gm_mode(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Num1) {
                self.gm_mode = GridmakerMode::AddEmpty;
            }
            if i.key_pressed(egui::Key::Num2) {
                self.gm_mode = GridmakerMode::AddBlocked;
            }
            if i.key_pressed(egui::Key::Num3) {
                self.gm_mode = GridmakerMode::AddStart;
            }
            if i.key_pressed(egui::Key::Num4) {
                self.gm_mode = GridmakerMode::AddEnd;
            }
        });
    }

    fn draw(&self, ui: &mut egui::Ui) {
        let painter = ui.painter();

        self.draw_grid(painter);
        self.draw_selected_square(painter);

        self.draw_status_bar(painter);
    }

    fn draw_grid(&self, painter: &egui::Painter) {
        // Draw squares
        for i in 0..self.grid.rows {
            for j in 0..self.grid.cols {
                let square = &self.grid.squares[i * self.grid.cols + j];
                let color: egui::Color32 = square.into();

                let rect = egui::Rect::from_min_max(
                    egui::pos2(
                        self.grid_area.min.x + j as f32 * self.grid.square_size as f32,
                        self.grid_area.min.y + i as f32 * self.grid.square_size as f32,
                    ),
                    egui::pos2(
                        self.grid_area.min.x + (j + 1) as f32 * self.grid.square_size as f32,
                        self.grid_area.min.y + (i + 1) as f32 * self.grid.square_size as f32,
                    ),
                );

                painter.rect_filled(rect, 0.0, color);
            }
        }

        // Draw grid lines
        // Horizontal
        for i in 0..self.grid.rows {
            let y = self.grid_area.min.y + i as f32 * self.grid.square_size as f32;
            let start = egui::pos2(self.grid_area.min.x, y);
            let end = egui::pos2(
                self.grid_area.min.x + self.grid.cols as f32 * self.grid.square_size as f32,
                y,
            );
            painter.line_segment(
                [start, end],
                egui::Stroke::new(GRID_LINE_WIDTH, GRID_LINE_COLOR),
            );
        }
        // Vertical
        for i in 0..self.grid.cols {
            let x = self.grid_area.min.x + i as f32 * self.grid.square_size as f32;
            let start = egui::pos2(x, self.grid_area.min.y);
            let end = egui::pos2(
                x,
                self.grid_area.min.y + self.grid.rows as f32 * self.grid.square_size as f32,
            );
            painter.line_segment(
                [start, end],
                egui::Stroke::new(GRID_LINE_WIDTH, GRID_LINE_COLOR),
            );
        }
    }

    fn draw_selected_square(&self, painter: &egui::Painter) {
        let min = egui::pos2(
            self.grid_area.min.x + (self.selected_square.0 * self.grid.square_size) as f32,
            self.grid_area.min.y + (self.selected_square.1 * self.grid.square_size) as f32,
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
    fn draw_status_bar(&self, painter: &egui::Painter) {
        let status_text = format!("Mode: {}\n(change with 1, 2, 3, 4)", self.gm_mode);

        painter.rect_filled(self.status_area, 0.0, egui::Color32::DARK_GRAY);
        painter.text(
            self.status_area.min
                + egui::vec2(
                    self.grid.square_size as f32,
                    self.status_area.height() / 2.0,
                ),
            egui::Align2::LEFT_CENTER,
            status_text,
            egui::FontId::monospace(20.0),
            egui::Color32::WHITE,
        );
    }
}

impl eframe::App for Pathfinder {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_state(ctx);
            self.draw(ui);
        });
    }
}
