use egui::Widget;

use crate::grid::Grid;

pub struct GridView<'a> {
    grid: &'a Grid,
    cell_px: f32,
}

impl<'a> GridView<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            cell_px: 24.0,
        }
    }
}

impl<'a> Widget for GridView<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let grid_width = self.grid.width();
        let grid_height = self.grid.height();
        let cell_size_px = self.cell_px;
        let grid_dimension = egui::vec2(
            grid_width as f32 * cell_size_px,
            grid_height as f32 * cell_size_px,
        );

        let (rect, response) = ui.allocate_exact_size(grid_dimension, egui::Sense::hover());
        let origin = rect.min;
        let painter = ui.painter_at(rect);

        // Draw alive cells
        for y in 0..grid_height {
            for x in 0..grid_width {
                if self.grid.get_cell_at_coord((x, y)).unwrap_or(false) {
                    let r = cell_rect(origin, cell_size_px, x, y);
                    painter.rect_filled(r, 0.0, ui.visuals().text_color());
                }
            }
        }

        // Draw strokes

        // Vertical
        let stroke = egui::Stroke::new(1.0, ui.visuals().weak_text_color());

        let right = rect.max.x - 0.5;
        for x in 0..=grid_width {
            let xa = (origin.x + x as f32 * cell_size_px).min(right);
            let ya = origin.y;

            let xb = (origin.x + x as f32 * cell_size_px).min(right);
            let yb = origin.y + grid_height as f32 * cell_size_px;

            painter.line_segment([egui::pos2(xa, ya), egui::pos2(xb, yb)], stroke);
        }

        // Horizontal
        let top = rect.min.y + 0.5;
        for y in 0..=grid_height {
            let xa = origin.x;
            let ya = (origin.y + y as f32 * cell_size_px).max(top);

            let xb = origin.x + grid_width as f32 * cell_size_px;
            let yb = (origin.y + y as f32 * cell_size_px).max(top);

            painter.line_segment([egui::pos2(xa, ya), egui::pos2(xb, yb)], stroke);
        }

        response
    }
}

fn cell_rect(origin: egui::Pos2, cell_size_px: f32, x: usize, y: usize) -> egui::Rect {
    let min = origin + egui::vec2(x as f32 * cell_size_px, y as f32 * cell_size_px);
    egui::Rect::from_min_size(min, egui::vec2(cell_size_px, cell_size_px))
}
