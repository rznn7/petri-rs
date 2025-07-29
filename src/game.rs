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
        let (new_cells, changed) = GridEvolver::next_generation(&self.grid)
            .expect("tick: computing next generation failed");

        self.grid
            .set_cells(new_cells)
            .expect("tick: applying new generation failed");
        self.generation += 1;
        changed
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "gen: {}", self.generation)?;
        write!(f, "{}", self.grid)
    }
}
