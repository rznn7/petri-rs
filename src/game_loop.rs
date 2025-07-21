use std::time::{Duration, Instant};

use crate::{game::Game, ui::PointerGridEvent};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum Playback {
    Playing,
    #[default]
    Paused,
}

#[derive(Debug)]
struct GameClock<T: TimeSource> {
    interval: Duration,
    last_tick: Option<Instant>,
    time: T,
}

pub trait TimeSource {
    fn now(&self) -> Instant;
}

#[derive(Default, Clone, Copy)]
pub struct SystemClock;

impl TimeSource for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

impl<T: TimeSource> GameClock<T> {
    fn new(interval: Duration, time: T) -> Self {
        Self {
            interval,
            last_tick: None,
            time,
        }
    }

    fn should_tick(&self) -> bool {
        self.last_tick
            .map(|t| self.time.now().duration_since(t) >= self.interval)
            .unwrap_or(false)
    }

    fn mark_tick(&mut self) {
        self.last_tick = Some(self.time.now());
    }

    fn set_interval(&mut self, d: Duration) {
        self.interval = d;
    }
}

pub struct GameController<T: TimeSource> {
    pub game: Game,
    playback: Playback,
    clock: GameClock<T>,
}

impl<T: TimeSource> GameController<T> {
    pub fn new(game: Game, time_source: T) -> Self {
        Self {
            game,
            playback: Playback::default(),
            clock: GameClock::new(Duration::from_millis(500), time_source),
        }
    }

    pub fn play(&mut self) {
        self.playback = Playback::Playing;
        self.tick();
    }

    pub fn pause(&mut self) {
        self.playback = Playback::Paused;
    }

    pub fn is_playing(&self) -> bool {
        self.playback == Playback::Playing
    }

    pub fn set_interval(&mut self, interval: Duration) {
        self.clock.set_interval(interval);
    }

    pub fn should_tick(&self) -> bool {
        self.is_playing() && self.clock.should_tick()
    }

    pub fn tick(&mut self) {
        let _changed = self.game.tick();
        self.clock.mark_tick();
    }

    pub fn handle_pointer_event(&mut self, event: PointerGridEvent) {
        match event {
            PointerGridEvent::Hovered { cell: _ } => {}
            PointerGridEvent::LeftClick { cell } => self.toggle_cell(cell),
            PointerGridEvent::RightClick { cell: _ } => {}
            PointerGridEvent::BothClick { cell: _ } => {}
        };
    }

    fn toggle_cell(&mut self, coord: (usize, usize)) {
        match self.game.grid.toggle_cell_at_coord(coord) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Error: Could not toggle");
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn playback_defaults_to_paused() {
        assert_eq!(Playback::default(), Playback::Paused);
    }

    #[derive(Clone, Copy)]
    pub struct MockClock {
        pub now: Instant,
    }
    impl TimeSource for MockClock {
        fn now(&self) -> Instant {
            self.now
        }
    }

    #[test]
    fn game_clock_triggers_after_interval() {
        let start = Instant::now();
        let mut clock = GameClock::new(Duration::from_millis(100), MockClock { now: start });

        clock.mark_tick();
        assert!(!clock.should_tick());

        clock.time.now += Duration::from_millis(99);
        assert!(!clock.should_tick());

        clock.time.now += Duration::from_millis(2);
        assert!(clock.should_tick());
    }

    #[test]
    fn test_should_handle_playing_state() {
        let mut gm = GameController::new(
            Game::new(Grid::new(3, 3)),
            MockClock {
                now: Instant::now(),
            },
        );

        assert!(!gm.is_playing());
        gm.play();
        assert!(gm.is_playing());
        gm.pause();
        assert!(!gm.is_playing());
    }

    #[test]
    fn test_game_clock_set_interval() {
        let mut clock = GameClock::new(
            Duration::from_secs(1),
            MockClock {
                now: Instant::now(),
            },
        );
        clock.set_interval(Duration::from_millis(250));
        assert_eq!(clock.interval, Duration::from_millis(250));
    }

    #[test]
    fn test_controller_should_tick_after_interval() {
        let start = Instant::now();
        let clock = MockClock { now: start };

        let mut controller = GameController::new(Game::new(Grid::new(1, 1)), clock);
        controller.play();

        assert!(!controller.should_tick());

        controller.clock.time.now += Duration::from_millis(500);
        assert!(controller.should_tick());

        controller.tick();
    }

    #[test]
    fn test_handle_pointer_event_hovered_does_nothing() {
        let mut controller = GameController::new(
            Game::new(Grid::new(2, 2)),
            MockClock {
                now: Instant::now(),
            },
        );
        let cell = (0, 0);
        assert!(!controller.game.grid.get_cell_at_coord(cell).unwrap());

        controller.handle_pointer_event(PointerGridEvent::Hovered { cell });

        assert!(!controller.game.grid.get_cell_at_coord(cell).unwrap());
    }

    #[test]
    fn test_handle_pointer_event_left_click_toggles_cell() {
        let mut controller = GameController::new(
            Game::new(Grid::new(3, 3)),
            MockClock {
                now: Instant::now(),
            },
        );

        let cell = (1, 1);
        assert!(!controller.game.grid.get_cell_at_coord(cell).unwrap());

        controller.handle_pointer_event(PointerGridEvent::LeftClick { cell });

        assert!(controller.game.grid.get_cell_at_coord(cell).unwrap());
    }

    #[test]
    fn test_handle_pointer_event_right_click_does_nothing() {
        let mut controller = GameController::new(
            Game::new(Grid::new(2, 2)),
            MockClock {
                now: Instant::now(),
            },
        );
        let cell = (0, 0);
        assert!(!controller.game.grid.get_cell_at_coord(cell).unwrap());

        controller.handle_pointer_event(PointerGridEvent::RightClick { cell });

        assert!(!controller.game.grid.get_cell_at_coord(cell).unwrap());
    }

    #[test]
    fn test_handle_pointer_event_both_click_does_nothing() {
        let mut controller = GameController::new(
            Game::new(Grid::new(2, 2)),
            MockClock {
                now: Instant::now(),
            },
        );
        let cell = (0, 0);
        assert!(!controller.game.grid.get_cell_at_coord(cell).unwrap());

        controller.handle_pointer_event(PointerGridEvent::BothClick { cell });

        assert!(!controller.game.grid.get_cell_at_coord(cell).unwrap());
    }
}
