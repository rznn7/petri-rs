use crate::grid::{Grid, IndexGridError};

pub struct GridEvolver;

impl GridEvolver {
    pub fn next_generation(grid: &Grid) -> Result<(Vec<bool>, bool), IndexGridError> {
        let cell_count = grid.width * grid.height;
        let mut next_cells = vec![false; cell_count];
        let mut has_changed = false;

        for (i, cell) in next_cells.iter_mut().enumerate() {
            let current_cell = grid.get_cell(i)?;
            let next_cell = Self::next_cell_state(grid, i)?;
            *cell = next_cell;

            if next_cell != current_cell {
                has_changed = true;
            }
        }

        Ok((next_cells, has_changed))
    }

    fn next_cell_state(grid: &Grid, i: usize) -> Result<bool, IndexGridError> {
        let current_state = grid.get_cell(i)?;
        let living_neighbors = grid.count_living_neighbors_at_coord(grid.index_to_coord(i))?;

        match (current_state, living_neighbors) {
            (false, 3) => Ok(true),
            (true, 2) => Ok(true),
            (true, 3) => Ok(true),
            _ => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_cell_dead_to_live() {
        let mut grid = Grid::new(3, 3);

        let _ = grid.set_cell_at_coord((0, 0), true);
        let _ = grid.set_cell_at_coord((0, 1), true);
        let _ = grid.set_cell_at_coord((0, 2), true);

        let result: Result<bool, IndexGridError> = GridEvolver::next_cell_state(&grid, 4);

        assert!(result.is_ok());
        if let Ok(value) = result {
            assert!(value);
        }
    }

    #[test]
    fn test_next_cell_dead_to_dead() {
        let grid = Grid::new(3, 3);

        let result: Result<bool, IndexGridError> = GridEvolver::next_cell_state(&grid, 4);

        assert!(result.is_ok());
        if let Ok(value) = result {
            assert!(!value);
        }
    }

    #[test]
    fn test_next_cell_live_to_dead() {
        let mut grid = Grid::new(3, 3);

        let _ = grid.set_cell_at_coord((1, 1), true);

        let result: Result<bool, IndexGridError> = GridEvolver::next_cell_state(&grid, 4);

        assert!(result.is_ok());
        if let Ok(value) = result {
            assert!(!value);
        }
    }

    #[test]
    fn test_next_cell_live_to_live() {
        let mut grid = Grid::new(3, 3);

        let _ = grid.set_cell_at_coord((1, 1), true);

        let _ = grid.set_cell_at_coord((0, 0), true);
        let _ = grid.set_cell_at_coord((0, 1), true);
        let _ = grid.set_cell_at_coord((0, 2), true);

        let result: Result<bool, IndexGridError> = GridEvolver::next_cell_state(&grid, 4);

        assert!(result.is_ok());
        if let Ok(value) = result {
            assert!(value);
        }
    }

    #[test]
    fn test_getting_next_generation() {
        let mut grid = Grid::new(3, 3);

        let _ = grid.set_cell_at_coord((0, 1), true);
        let _ = grid.set_cell_at_coord((1, 1), true);
        let _ = grid.set_cell_at_coord((2, 1), true);

        let result = GridEvolver::next_generation(&grid);
        assert!(result.is_ok());
        if let Ok((new_cells, changed)) = result {
            assert!(changed);
            assert_eq!(
                new_cells,
                vec![false, true, false, false, true, false, false, true, false]
            )
        }
    }
}
