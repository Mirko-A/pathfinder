use crate::algorithms::grid;

const EMPTY_SQUARE_COLOR: egui::Color32 = egui::Color32::WHITE;
const BLOCKED_SQUARE_COLOR: egui::Color32 = egui::Color32::BLACK;
const START_SQUARE_COLOR: egui::Color32 = egui::Color32::GREEN;
const END_SQUARE_COLOR: egui::Color32 = egui::Color32::RED;
const SELECTED_SQUARE_COLOR: egui::Color32 = egui::Color32::DARK_BLUE;
const GRID_LINE_COLOR: egui::Color32 = egui::Color32::BLACK;

impl From<&grid::Square> for egui::Color32 {
    fn from(state: &grid::Square) -> egui::Color32 {
        match state {
            grid::Square::Empty => EMPTY_SQUARE_COLOR,
            grid::Square::Blocked => BLOCKED_SQUARE_COLOR,
            grid::Square::Start => START_SQUARE_COLOR,
            grid::Square::End => END_SQUARE_COLOR,
        }
    }
}

struct WindowProps {
    title: String,
    width: f32,
    height: f32,
}

const APP_TITLE: &str = "Pathfinder";
const DEFAULT_SQUARE_SIZE: usize = 25;

pub struct Pathfinder {
    win_props: WindowProps,
    grid_area: egui::Rect,
    status_area: egui::Rect,
    selected_square: grid::Point,
    placing_square: Option<grid::Square>,
    square_size: usize,
    grid: grid::Grid,
}

impl Default for Pathfinder {
    fn default() -> Self {
        let grid = grid::Grid::default();
        Self::from_grid(grid, DEFAULT_SQUARE_SIZE)
    }
}

const SELECTED_SQUARE_WIDTH: f32 = 3.0;
const GRID_LINE_WIDTH: f32 = 1.0;

impl Pathfinder {
    pub fn new(rows: usize, cols: usize, square_size: usize, randomize: bool) -> Self {
        let grid = if randomize {
            grid::Grid::with_random_blockage(rows, cols)
        } else {
            grid::Grid::new(rows, cols)
        };

        Self::from_grid(grid, square_size)
    }

    fn from_grid(grid: grid::Grid, square_size: usize) -> Self {
        let grid_width = grid.cols as f32 * square_size as f32;
        let grid_height = grid.rows as f32 * square_size as f32;

        let grid_area =
            egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(grid_width, grid_height));

        let status_area_height = square_size as f32 * 3.0;
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
            selected_square: grid::Point::new(0, 0),
            placing_square: None,
            square_size,
            grid,
        }
    }

    pub fn run(app: Self) -> eframe::Result<()> {
        let native_options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(app.win_props.width, app.win_props.height)),
            resizable: false,
            ..Default::default()
        };
        eframe::run_native(
            app.win_props.title.clone().as_str(),
            native_options,
            Box::new(|_cc| Box::new(app)),
        )
    }

    fn update_state(&mut self, ctx: &egui::Context) {
        self.update_selected_square(ctx);
        self.update_grid(ctx);
        self.update_placing_square(ctx);
    }

    fn update_selected_square(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if (i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::H))
                && self.selected_square.row != 0
            {
                self.selected_square.row -= 1;
            }
            if (i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::L))
                && self.selected_square.row < self.grid.cols - 1
            {
                self.selected_square.row += 1;
            }
            if (i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::K))
                && self.selected_square.col != 0
            {
                self.selected_square.col -= 1;
            }
            if (i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::J))
                && self.selected_square.col < self.grid.rows - 1
            {
                self.selected_square.col += 1;
            }
        });
    }

    fn update_grid(&mut self, ctx: &egui::Context) {
        let idx = self.selected_square.col * self.grid.cols + self.selected_square.row;
        if let Some(ref placing_square) = self.placing_square {
            match placing_square {
                grid::Square::Empty | grid::Square::Blocked => {
                    if let Some(pos) = self.grid.start {
                        if pos == self.selected_square {
                            self.grid.start = None;
                        }
                    }
                    if let Some(pos) = self.grid.end {
                        if pos == self.selected_square {
                            self.grid.end = None;
                        }
                    }

                    if *placing_square == grid::Square::Empty {
                        self.grid.squares[idx] = grid::Square::Empty;
                    } else {
                        self.grid.squares[idx] = grid::Square::Blocked;
                    }
                }
                grid::Square::Start => {
                    if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
                        if let Some(pos) = self.grid.start {
                            self.grid.squares[pos.col * self.grid.cols + pos.row] =
                                grid::Square::Empty;
                        }
                        self.grid.start = Some(self.selected_square);
                        self.grid.squares[idx] = grid::Square::Start;
                    }
                }
                grid::Square::End => {
                    if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
                        if let Some(pos) = self.grid.end {
                            self.grid.squares[pos.col * self.grid.cols + pos.row] =
                                grid::Square::Empty;
                        }
                        self.grid.end = Some(self.selected_square);
                        self.grid.squares[idx] = grid::Square::End;
                    }
                }
            }
        }
    }

    fn update_placing_square(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Num1) {
                self.placing_square = Some(grid::Square::Empty);
            }
            if i.key_pressed(egui::Key::Num2) {
                self.placing_square = Some(grid::Square::Blocked);
            }
            if i.key_pressed(egui::Key::Num3) {
                self.placing_square = Some(grid::Square::Start);
            }
            if i.key_pressed(egui::Key::Num4) {
                self.placing_square = Some(grid::Square::End);
            }
            if i.key_pressed(egui::Key::Escape) {
                self.placing_square = None;
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
                        self.grid_area.min.x + j as f32 * self.square_size as f32,
                        self.grid_area.min.y + i as f32 * self.square_size as f32,
                    ),
                    egui::pos2(
                        self.grid_area.min.x + (j + 1) as f32 * self.square_size as f32,
                        self.grid_area.min.y + (i + 1) as f32 * self.square_size as f32,
                    ),
                );

                painter.rect_filled(rect, 0.0, color);
            }
        }

        // Draw grid lines
        // Horizontal
        for i in 0..self.grid.rows {
            let y = self.grid_area.min.y + i as f32 * self.square_size as f32;
            let start = egui::pos2(self.grid_area.min.x, y);
            let end = egui::pos2(
                self.grid_area.min.x + self.grid.cols as f32 * self.square_size as f32,
                y,
            );
            painter.line_segment(
                [start, end],
                egui::Stroke::new(GRID_LINE_WIDTH, GRID_LINE_COLOR),
            );
        }
        // Vertical
        for i in 0..self.grid.cols {
            let x = self.grid_area.min.x + i as f32 * self.square_size as f32;
            let start = egui::pos2(x, self.grid_area.min.y);
            let end = egui::pos2(
                x,
                self.grid_area.min.y + self.grid.rows as f32 * self.square_size as f32,
            );
            painter.line_segment(
                [start, end],
                egui::Stroke::new(GRID_LINE_WIDTH, GRID_LINE_COLOR),
            );
        }
    }

    fn draw_selected_square(&self, painter: &egui::Painter) {
        let min = egui::pos2(
            self.grid_area.min.x + (self.selected_square.row * self.square_size) as f32,
            self.grid_area.min.y + (self.selected_square.col * self.square_size) as f32,
        );
        let size = egui::vec2(self.square_size as f32, self.square_size as f32);
        let select_rect = egui::Rect::from_min_size(min, size);
        painter.rect(
            select_rect,
            0.0,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(SELECTED_SQUARE_WIDTH, SELECTED_SQUARE_COLOR),
        );
    }
    fn draw_status_bar(&self, painter: &egui::Painter) {
        let placing_square = match self.placing_square {
            None => "None".to_string(),
            Some(ref square) => match square {
                grid::Square::Start | grid::Square::End => format!("{} square (use Space)", square),
                _ => format!("{} square", square),
            },
        };
        let status_text = format!(
            "Placing: {}\n\nChange with 1, 2, 3, 4; ESC to cancel",
            placing_square
        );

        painter.rect_filled(self.status_area, 0.0, egui::Color32::DARK_GRAY);
        // Draw text in the center of the status area (vertically) and
        // with one square worth of padding from the left.
        let offset = egui::vec2(self.square_size as f32, self.status_area.height() / 2.0);
        painter.text(
            self.status_area.min + offset,
            egui::Align2::LEFT_CENTER,
            status_text,
            egui::FontId::proportional((self.status_area.height() * 4.0).sqrt()),
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
