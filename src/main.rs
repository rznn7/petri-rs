use std::{thread, time::Duration};

use game::Game;
use index_grid::IndexGrid;

mod game;
mod index_grid;

fn main() {
    let mut game = Game::new(IndexGrid::new(5, 5));

    let _ = game.grid.set_cell_at_coord((2, 1), true);
    let _ = game.grid.set_cell_at_coord((2, 2), true);
    let _ = game.grid.set_cell_at_coord((2, 3), true);

    println!("{}", game);
    thread::sleep(Duration::from_millis(500));

    while game.tick() {
        println!("{}", game);
        thread::sleep(Duration::from_millis(500));
    }
}
