use std::time::{Duration, Instant};

use crate::game::Game;

pub struct GameManager {
    pub game: Game,
    is_playing: bool,
    interval: Duration,
    last_tick_time: Option<Instant>,
}

impl GameManager {
    pub fn new(game: Game) -> Self {
        GameManager {
            game,
            is_playing: false,
            interval: Duration::from_millis(500),
            last_tick_time: None,
        }
    }

    pub fn tick(&mut self) {
        let _has_changed = self.game.tick();
        self.last_tick_time = Some(Instant::now());
    }

    pub fn should_tick(&mut self) -> bool {
        if !self.is_playing {
            return false;
        }

        if let Some(last_tick) = self.last_tick_time {
            Instant::now().duration_since(last_tick) >= self.interval
        } else {
            false
        }
    }

    pub fn play(&mut self) {
        self.is_playing = true;
        self.tick();
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
        self.last_tick_time = None;
    }

    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::index_grid::IndexGrid;

    #[test]
    fn test_should_create_game_manager_with_game() {
        let game_manager = GameManager::new(Game::new(IndexGrid::new(3, 3)));

        let _game = game_manager.game;
    }

    #[test]
    fn test_should_allow_tick_trigger() {
        let mut game_manager = GameManager::new(Game::new(IndexGrid::new(3, 3)));

        game_manager.game.tick();
    }

    #[test]
    fn test_should_handle_playing_state() {
        let mut game_manager = GameManager::new(Game::new(IndexGrid::new(3, 3)));

        assert!(!game_manager.is_playing());
        game_manager.play();
        assert!(game_manager.is_playing());
        game_manager.pause();
        assert!(!game_manager.is_playing());
    }

    #[test]
    fn test_should_have_interval() {
        let mut game_manager = GameManager::new(Game::new(IndexGrid::new(3, 3)));

        assert_eq!(game_manager.interval, Duration::from_millis(500));
        game_manager.set_interval(Duration::from_millis(200));
        assert_eq!(game_manager.interval, Duration::from_millis(200));
    }

    #[test]
    fn test_should_track_last_tick_time() {
        let mut game_manager = GameManager::new(Game::new(IndexGrid::new(3, 3)));

        assert!(game_manager.last_tick_time.is_none());

        let before_tick = Instant::now();
        game_manager.tick();
        let after_tick = Instant::now();

        let last_tick = game_manager.last_tick_time.unwrap();
        assert!(last_tick >= before_tick);
        assert!(last_tick <= after_tick);
    }

    #[test]
    fn test_should_ask_for_tick_if_interval_passed() {
        let mut game_manager = GameManager {
            game: Game::new(IndexGrid::new(1, 1)),
            interval: Duration::from_millis(100),
            is_playing: true,
            last_tick_time: Instant::now().checked_sub(Duration::from_millis(500)),
        };

        assert!(game_manager.should_tick());
    }

    #[test]
    fn test_should_not_ask_for_tick_if_interval_not_passed() {
        let mut game_manager = GameManager {
            game: Game::new(IndexGrid::new(1, 1)),
            interval: Duration::from_secs(500),
            is_playing: true,
            last_tick_time: Some(Instant::now()),
        };

        assert!(!game_manager.should_tick());
    }
}
