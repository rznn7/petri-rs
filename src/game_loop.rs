use std::time::{Duration, Instant};

use crate::game::Game;

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

trait TimeSource {
    fn now(&self) -> Instant;
}

#[derive(Default, Clone, Copy)]
struct SystemClock;

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

pub struct GameManager {
    pub game: Game,
    playback: Playback,
    clock: GameClock<SystemClock>,
}

impl GameManager {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            playback: Playback::default(),
            clock: GameClock::new(Duration::from_millis(500), SystemClock),
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
        let mut gm = GameManager::new(Game::new(Grid::new(3, 3)));

        assert!(!gm.is_playing());
        gm.play();
        assert!(gm.is_playing());
        gm.pause();
        assert!(!gm.is_playing());
    }

    #[test]
    fn test_should_tick_after_interval() {
        let mut gm = GameManager::new(Game::new(Grid::new(1, 1)));
        gm.play();
        gm.set_interval(Duration::from_millis(1));

        while !gm.should_tick() {}
        gm.tick();
    }
}
