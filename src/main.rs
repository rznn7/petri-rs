use std::time::Duration;

use eframe::egui;
use game::Game;
use game_loop::{GameController, SystemClock};
use grid::Grid;
use ui::GridView;

mod game;
mod game_loop;
mod grid;
mod grid_evolver;
mod ui;

fn main() -> eframe::Result {
    eframe::run_native(
        "petri-rs",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default(),
            ..Default::default()
        },
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    game_controller: GameController<SystemClock>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut grid = Grid::new(3, 3);
        let _ = grid.set_cell_at_coord((0, 1), true);
        let _ = grid.set_cell_at_coord((1, 1), true);
        let _ = grid.set_cell_at_coord((2, 1), true);

        let mut game_controller = GameController::new(Game::new(grid), SystemClock);
        game_controller.set_interval(Duration::from_millis(200));

        Self { game_controller }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.game_controller.is_playing() {
            ctx.request_repaint();
        }

        if self.game_controller.should_tick() {
            self.game_controller.tick();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.game_controller.is_playing() {
                    if ui.button("⏸").clicked() {
                        self.game_controller.pause();
                    }
                } else if ui.button("⏵").clicked() {
                    self.game_controller.play();
                }

                if ui.button("⏭").clicked() {
                    self.game_controller.game.tick();
                }
            });

            let result = GridView::new(&self.game_controller.game.grid).show(ui);

            if let Some(event) = result.pointer_event {
                self.game_controller.handle_pointer_event(event);
            }
        });
    }
}
