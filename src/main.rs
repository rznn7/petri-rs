use std::{thread, time::Duration};

use game::Game;
use grid::{Coord, Grid};

mod game;
mod grid;

fn main() {
    let mut game = Game::new(Grid::new(5, 5));

    let _ = game.grid.set_cell_state(&Coord { x: 2, y: 1 }, true);
    let _ = game.grid.set_cell_state(&Coord { x: 2, y: 2 }, true);
    let _ = game.grid.set_cell_state(&Coord { x: 2, y: 3 }, true);

    println!("{}", game);
    thread::sleep(Duration::from_millis(500));

    while game.tick() {
        println!("{}", game);
        thread::sleep(Duration::from_millis(500));
    }
}
