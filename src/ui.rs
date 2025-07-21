use crate::grid::Grid;

pub struct GridView<'a> {
    grid: &'a Grid,
    cell_px: f32,
}

pub struct GridViewResult {
    pub response: egui::Response,
    pub pointer_event: Option<PointerGridEvent>,
}

#[derive(Debug)]
pub enum PointerGridEvent {
    Hovered { cell: (usize, usize) },
    LeftClick { cell: (usize, usize) },
    RightClick { cell: (usize, usize) },
    BothClick { cell: (usize, usize) },
}

impl<'a> GridView<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            cell_px: 24.0,
        }
    }

    pub fn show(self, ui: &mut egui::Ui) -> GridViewResult {
        let grid_width = self.grid.width();
        let grid_height = self.grid.height();
        let cell_size_px = self.cell_px;
        let grid_dimension = egui::vec2(
            grid_width as f32 * cell_size_px,
            grid_height as f32 * cell_size_px,
        );

        let (rect, response) =
            ui.allocate_exact_size(grid_dimension, egui::Sense::click_and_drag());
        let origin = rect.min;
        let painter = ui.painter_at(rect);

        let pointer_event = if response.hovered() {
            ui.input(|i| {
                i.pointer.hover_pos().map(|pointer_pos| {
                    let local_position = pointer_pos - origin;
                    let cell = (
                        (local_position.x / cell_size_px).floor() as usize,
                        (local_position.y / cell_size_px).floor() as usize,
                    );

                    let left = i.pointer.button_pressed(egui::PointerButton::Primary);
                    let right = i.pointer.button_pressed(egui::PointerButton::Secondary);

                    match (left, right) {
                        (true, true) => PointerGridEvent::BothClick { cell },
                        (true, false) => PointerGridEvent::LeftClick { cell },
                        (false, true) => PointerGridEvent::RightClick { cell },
                        (false, false) => PointerGridEvent::Hovered { cell },
                    }
                })
            })
        } else {
            None
        };

        // Draw alive cells and hovered/clicked cell
        for y in 0..grid_height {
            for x in 0..grid_width {
                let cell_alive = self.grid.get_cell_at_coord((x, y)).unwrap_or(false);
                let cell_hovered = match pointer_event {
                    Some(PointerGridEvent::Hovered { cell })
                    | Some(PointerGridEvent::LeftClick { cell })
                    | Some(PointerGridEvent::RightClick { cell })
                    | Some(PointerGridEvent::BothClick { cell }) => cell == (x, y),
                    None => false,
                };

                if cell_alive || cell_hovered {
                    let r = cell_rect(origin, cell_size_px, x, y);
                    let color = if cell_alive {
                        ui.visuals().text_color()
                    } else {
                        ui.visuals().weak_text_color()
                    };
                    painter.rect_filled(r, 0.0, color);
                }
            }
        }

        // Draw grid lines
        let stroke = egui::Stroke::new(1.0, ui.visuals().weak_text_color());

        let right = rect.max.x - 0.5;
        for x in 0..=grid_width {
            let xa = (origin.x + x as f32 * cell_size_px).min(right);
            let ya = origin.y;
            let yb = origin.y + grid_height as f32 * cell_size_px;

            painter.line_segment([egui::pos2(xa, ya), egui::pos2(xa, yb)], stroke);
        }

        let top = rect.min.y + 0.5;
        for y in 0..=grid_height {
            let ya = (origin.y + y as f32 * cell_size_px).max(top);
            let xb = origin.x + grid_width as f32 * cell_size_px;

            painter.line_segment([egui::pos2(origin.x, ya), egui::pos2(xb, ya)], stroke);
        }

        GridViewResult {
            response,
            pointer_event,
        }
    }
}

fn cell_rect(origin: egui::Pos2, cell_size_px: f32, x: usize, y: usize) -> egui::Rect {
    let min = origin + egui::vec2(x as f32 * cell_size_px, y as f32 * cell_size_px);
    egui::Rect::from_min_size(min, egui::vec2(cell_size_px, cell_size_px))
}
