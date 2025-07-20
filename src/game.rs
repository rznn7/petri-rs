use std::fmt::Display;

use crate::{grid::Grid, grid_evolver::GridEvolver};

pub struct Game {
    generation: u32,
    pub grid: Grid,
}

impl Game {
    pub fn new(grid: Grid) -> Self {
        Game {
            generation: 0,
            grid,
        }
    }

    pub fn tick(&mut self) -> bool {
        match GridEvolver::next_generation(&self.grid) {
            Ok(res) => {
                self.grid.cells = res.0;
                self.generation += 1;
                res.1
            }
            _ => {
                panic!("Could not compute next generation.");
            }
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "gen: {}", self.generation)?;
        write!(f, "{}", self.grid)
    }
}
