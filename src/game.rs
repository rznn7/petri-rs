use crate::grid::Grid;

pub struct Game {
    generation: u32,
    pub grid: Grid,
    size: (usize, usize),
}

impl Game {
    pub fn new(size: (usize, usize)) -> Self {
        Game {
            generation: 0,
            grid: Grid::new(size.0, size.1),
            size,
        }
    }

    pub fn tick(&mut self) -> bool {
        let current_grid = &self.grid;
        let mut new_grid = self.grid.clone();

        let mut has_changed = false;

        current_grid.get_all_coords().iter().for_each(|coord| {
            let current_state = current_grid.cell_state(coord).unwrap_or(false);
            let next_state = current_grid.next_cell_state(coord);

            if current_state != next_state {
                let _ = new_grid.set_cell_state(coord, next_state);
                has_changed = true
            }
        });

        self.grid = new_grid;

        has_changed
    }
}
