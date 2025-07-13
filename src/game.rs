use std::fmt::Display;

use crate::index_grid::IndexGrid;

pub struct Game {
    generation: u32,
    pub grid: IndexGrid,
}

impl Game {
    pub fn new(grid: IndexGrid) -> Self {
        Game {
            generation: 0,
            grid,
        }
    }

    pub fn tick(&mut self) -> bool {
        match self.grid.next_cells_with_change_info() {
            Ok(res) => {
                self.grid.cells = res.0;
                self.generation += 1;
                res.1
            }
            _ => {
                panic!("Could not compute next ");
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
