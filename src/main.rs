use std::time::Duration;

use eframe::egui;
use game::Game;
use game_manager::GameManager;
use grid::Grid;

mod game;
mod game_manager;
mod grid;

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
    game_manager: GameManager,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut grid = Grid::new(3, 3);
        let _ = grid.set_cell_at_coord((0, 1), true);
        let _ = grid.set_cell_at_coord((1, 1), true);
        let _ = grid.set_cell_at_coord((2, 1), true);

        let mut game_manager = GameManager::new(Game::new(grid));
        game_manager.set_interval(Duration::from_millis(200));

        Self { game_manager }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.game_manager.is_playing() {
            ctx.request_repaint();
        }

        if self.game_manager.should_tick() {
            self.game_manager.tick();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.game_manager.is_playing() {
                    if ui.button("⏸").clicked() {
                        self.game_manager.pause();
                    }
                } else if ui.button("⏵").clicked() {
                    self.game_manager.play();
                }

                if ui.button("⏭").clicked() {
                    self.game_manager.game.tick();
                }
            });

            ui.monospace(self.game_manager.game.to_string());
        });
    }
}
