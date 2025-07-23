use std::time::Duration;

use game::Game;
use game_loop::{GameController, SystemClock};
use grid::Grid;
use ui::GridView;

mod game;
mod game_loop;
mod grid;
mod grid_evolver;
mod ui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "petri-rs",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([200.0, 120.0]),
            ..Default::default()
        },
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    screen: AppScreen,
    toasts: egui_notify::Toasts,
}

enum AppScreen {
    Setup(SetupState),
    Playing(GameState),
}

struct SetupState {
    width: String,
    height: String,
}

struct GameState {
    controller: GameController<SystemClock>,
    scroll_offset: egui::Vec2,
    zoom: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            screen: AppScreen::Setup(SetupState {
                width: "400".into(),
                height: "200".into(),
            }),
            toasts: egui_notify::Toasts::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match &mut self.screen {
            AppScreen::Setup(_) => self.render_setup_ui(ui, ctx),
            AppScreen::Playing(_) => self.render_game_ui(ui, ctx),
        });

        self.toasts.show(ctx);
    }
}

impl MyApp {
    fn render_setup_ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let AppScreen::Setup(setup) = &mut self.screen else {
            return;
        };

        ui.label(egui::RichText::new("petri-rs").heading());
        ui.label("Setup grid");

        ui.horizontal(|ui| {
            ui.label("Width:");
            ui.text_edit_singleline(&mut setup.width);
        });

        ui.horizontal(|ui| {
            ui.label("Height:");
            ui.text_edit_singleline(&mut setup.height);
        });

        if ui.button("Start").clicked() {
            let width: usize = match setup.width.parse() {
                Ok(w) if w > 0 => w,
                _ => {
                    self.toasts.warning("Width must be a positive number!");
                    return;
                }
            };

            let height: usize = match setup.height.parse() {
                Ok(h) if h > 0 && h <= 200 => h,
                _ => {
                    self.toasts.warning("Height must be a positive number!");
                    return;
                }
            };

            if height > 300 {
                self.toasts.warning("Max height is 300!");
                return;
            }

            if width > 500 {
                self.toasts.warning("Max width is 500!");
                return;
            }

            self.screen = AppScreen::Playing(GameState {
                controller: GameController::new(Game::new(Grid::new(width, height)), SystemClock)
                    .with_interval(Duration::from_millis(100)),
                scroll_offset: egui::Vec2::ZERO,
                zoom: 1.0,
            });

            ctx.set_pixels_per_point(1.0);
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([1280.0, 720.0].into()));
        }
    }

    fn render_game_ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let AppScreen::Playing(game) = &mut self.screen else {
            return;
        };

        if game.controller.is_playing() {
            ctx.request_repaint();
        }

        if game.controller.should_tick() {
            game.controller.tick();
        }

        let (scroll, ctrl) = ctx.input(|i| (i.raw_scroll_delta.y, i.modifiers.ctrl));
        if scroll != 0.0 && ctrl {
            let zoom_speed = 0.01;
            game.zoom = (game.zoom + scroll * zoom_speed).clamp(0.5, 4.0);
        }

        ui.horizontal(|ui| {
            if game.controller.is_playing() {
                if ui.button("⏸").clicked() {
                    game.controller.pause();
                }
            } else if ui.button("⏵").clicked() {
                game.controller.play();
            }

            if ui.button("⏭").clicked() {
                game.controller.game.tick();
            }
        });

        egui::ScrollArea::both()
            .scroll_offset(game.scroll_offset)
            .show(ui, |ui| {
                let cell_size = 14.0 * game.zoom;
                let result = GridView::new(&game.controller.game.grid, cell_size).show(ui);

                if let Some(event) = result.pointer_event {
                    game.controller.handle_pointer_event(event);
                }

                if ui.ctx().input(|i| i.pointer.middle_down()) {
                    let delta = ui.ctx().input(|i| i.pointer.delta());
                    game.scroll_offset -= delta;
                }
            });
    }
}
