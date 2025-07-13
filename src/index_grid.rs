use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct IndexGrid {
    pub cells: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl IndexGrid {
    pub fn new(width: usize, height: usize) -> Self {
        IndexGrid {
            cells: vec![false; width * height],
            width,
            height,
        }
    }

    fn get_cell(&self, i: usize) -> Result<bool, IndexGridError> {
        if !self.is_index_inbounds(i) {
            return Err(IndexGridError::IndexOutOfBounds);
        }

        Ok(self.cells[i])
    }

    fn get_cell_at_coord(&self, coord: (usize, usize)) -> Result<bool, IndexGridError> {
        self.get_cell(self.coord_to_index(coord))
    }

    fn set_cell(&mut self, i: usize, value: bool) -> Result<(), IndexGridError> {
        if !self.is_index_inbounds(i) {
            return Err(IndexGridError::IndexOutOfBounds);
        }

        self.cells[i] = value;
        Ok(())
    }

    pub fn set_cell_at_coord(
        &mut self,
        coord: (usize, usize),
        value: bool,
    ) -> Result<(), IndexGridError> {
        self.set_cell(self.coord_to_index(coord), value)
    }

    fn index_to_coord(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    fn coord_to_index(&self, coord: (usize, usize)) -> usize {
        let (x, y) = coord;
        y * self.width + x
    }

    fn count_living_neighbors_at_coord(
        &self,
        coord: (usize, usize),
    ) -> Result<usize, IndexGridError> {
        if !self.is_index_inbounds(self.coord_to_index(coord)) {
            return Err(IndexGridError::IndexOutOfBounds);
        }

        let (x, y) = (coord.0 as i32, coord.1 as i32);

        let potential_neighbors = [
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y - 1),
            (x + 1, y + 1),
            (x + 1, y - 1),
        ];

        let valid_neighbors: Vec<(usize, usize)> = potential_neighbors
            .iter()
            .filter(|&&n| self.is_coord_inbounds(n))
            .map(|&n| (n.0 as usize, n.1 as usize))
            .collect();

        let living_neighbors_count = valid_neighbors
            .iter()
            .map(|&n| self.get_cell_at_coord(n).unwrap_or(false))
            .filter(|&v| v)
            .count();

        Ok(living_neighbors_count)
    }

    fn is_index_inbounds(&self, i: usize) -> bool {
        i < self.width * self.height
    }

    fn is_coord_inbounds(&self, coord: (i32, i32)) -> bool {
        let (x, y) = coord;

        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }

    fn next_cell(&self, i: usize) -> Result<bool, IndexGridError> {
        let current_state = self.get_cell(i)?;
        let living_neighbors = self.count_living_neighbors_at_coord(self.index_to_coord(i))?;

        match (current_state, living_neighbors) {
            (false, 3) => Ok(true),
            (true, 2) => Ok(true),
            (true, 3) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn next_cells_with_change_info(&self) -> Result<(Vec<bool>, bool), IndexGridError> {
        let cell_count = self.width * self.height;
        let mut next_cells = vec![false; cell_count];
        let mut has_changed = false;

        for (i, cell) in next_cells.iter_mut().enumerate() {
            let current_cell = self.get_cell(i)?;
            let next_cell = self.next_cell(i)?;
            *cell = next_cell;

            if next_cell != current_cell {
                has_changed = true;
            }
        }

        Ok((next_cells, has_changed))
    }
}

#[derive(Clone, Debug)]
pub enum IndexGridError {
    IndexOutOfBounds,
}

impl Display for IndexGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &cell) in self.cells.iter().enumerate() {
            if i % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, " {} ", get_symbol(cell))?;
        }
        Ok(())
    }
}

fn get_symbol(value: bool) -> &'static str {
    if value { "■" } else { "•" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_empty_grid() {
        let grid = IndexGrid::new(4, 5);

        for i in 0..20 {
            if let Ok(value) = grid.get_cell(i) {
                assert!(!value);
            }
        }
    }

    #[test]
    fn test_accessing_cell_out_of_bounds() {
        let grid = IndexGrid::new(1, 1);

        let result: Result<bool, IndexGridError> = grid.get_cell(100);

        assert!(result.is_err());
    }

    #[test]
    fn test_setting_cell() {
        let mut grid = IndexGrid::new(5, 5);

        let result: Result<(), IndexGridError> = grid.set_cell(5, true);

        assert!(result.is_ok());

        if let Ok(value) = grid.get_cell(5) {
            assert!(value);
        }
    }

    #[test]
    fn test_setting_cell_out_of_bounds() {
        let mut grid = IndexGrid::new(1, 1);

        let result: Result<(), IndexGridError> = grid.set_cell(100, true);

        assert!(result.is_err());
    }

    #[test]
    fn test_getting_indexes_as_coordinates() {
        let grid = IndexGrid::new(4, 4);

        assert_eq!(grid.index_to_coord(0), (0, 0));
        assert_eq!(grid.index_to_coord(1), (1, 0));
        assert_eq!(grid.index_to_coord(3), (3, 0));
        assert_eq!(grid.index_to_coord(7), (3, 1));
        assert_eq!(grid.index_to_coord(8), (0, 2));
        assert_eq!(grid.index_to_coord(9), (1, 2));
        assert_eq!(grid.index_to_coord(12), (0, 3));
        assert_eq!(grid.index_to_coord(14), (2, 3));
        assert_eq!(grid.index_to_coord(15), (3, 3));
    }

    #[test]
    fn test_getting_coordinates_as_indexes() {
        let grid = IndexGrid::new(4, 4);

        assert_eq!(grid.coord_to_index((0, 0)), 0);
        assert_eq!(grid.coord_to_index((1, 0)), 1);
        assert_eq!(grid.coord_to_index((3, 0)), 3);
        assert_eq!(grid.coord_to_index((3, 1)), 7);
        assert_eq!(grid.coord_to_index((0, 2)), 8);
        assert_eq!(grid.coord_to_index((1, 2)), 9);
        assert_eq!(grid.coord_to_index((0, 3)), 12);
        assert_eq!(grid.coord_to_index((2, 3)), 14);
        assert_eq!(grid.coord_to_index((3, 3)), 15);
    }

    #[test]
    fn test_getting_cell_with_coord() {
        let mut grid = IndexGrid::new(4, 4);

        let _ = grid.set_cell(15, true);

        let result: Result<bool, IndexGridError> = grid.get_cell_at_coord((3, 3));
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert!(value);
        }
    }

    #[test]
    fn test_setting_cell_with_coord() {
        let mut grid = IndexGrid::new(5, 5);

        let result: Result<(), IndexGridError> = grid.set_cell_at_coord((4, 3), true);
        assert!(result.is_ok());
        if let Ok(value) = grid.get_cell_at_coord((4, 3)) {
            assert!(value);
        }
    }

    #[test]
    fn test_count_living_neighbors() {
        let mut grid = IndexGrid::new(3, 3);

        let _ = grid.set_cell_at_coord((1, 0), true);
        let _ = grid.set_cell_at_coord((0, 1), true);
        let _ = grid.set_cell_at_coord((2, 1), true);

        let result: Result<usize, IndexGridError> = grid.count_living_neighbors_at_coord((1, 1));

        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, 3);
        }
    }

    #[test]
    fn test_count_living_neighbors_in_corners() {
        let mut grid = IndexGrid::new(3, 3);

        let _ = grid.set_cell_at_coord((1, 1), true);

        [
            grid.count_living_neighbors_at_coord((0, 0)),
            grid.count_living_neighbors_at_coord((0, 2)),
            grid.count_living_neighbors_at_coord((2, 0)),
            grid.count_living_neighbors_at_coord((2, 2)),
        ]
        .into_iter()
        .for_each(|result| {
            assert!(result.is_ok());
            if let Ok(value) = result {
                assert_eq!(value, 1);
            }
        });
    }

    #[test]
    fn test_cell_next_dead_to_live() {
        let mut grid = IndexGrid::new(3, 3);

        let _ = grid.set_cell_at_coord((0, 0), true);
        let _ = grid.set_cell_at_coord((0, 1), true);
        let _ = grid.set_cell_at_coord((0, 2), true);

        let result: Result<bool, IndexGridError> = grid.next_cell(4);

        assert!(result.is_ok());
        if let Ok(value) = result {
            assert!(value);
        }
    }

    #[test]
    fn test_cell_next_dead_to_dead() {
        let grid = IndexGrid::new(3, 3);

        let result: Result<bool, IndexGridError> = grid.next_cell(4);

        assert!(result.is_ok());
        if let Ok(value) = result {
            assert!(!value);
        }
    }

    #[test]
    fn test_cell_live_to_dead() {
        let mut grid = IndexGrid::new(3, 3);

        let _ = grid.set_cell(4, true);

        let result: Result<bool, IndexGridError> = grid.next_cell(4);

        assert!(result.is_ok());
        if let Ok(value) = result {
            assert!(!value);
        }
    }

    #[test]
    fn test_cell_next_live_to_live() {
        let mut grid = IndexGrid::new(3, 3);

        let _ = grid.set_cell(4, true);

        let _ = grid.set_cell_at_coord((0, 0), true);
        let _ = grid.set_cell_at_coord((0, 1), true);
        let _ = grid.set_cell_at_coord((0, 2), true);

        let result: Result<bool, IndexGridError> = grid.next_cell(4);

        assert!(result.is_ok());
        if let Ok(value) = result {
            assert!(value);
        }
    }

    #[test]
    fn test_getting_grid_next_state_with_change_info() {
        let mut grid = IndexGrid::new(3, 3);

        let _ = grid.set_cell(3, true);
        let _ = grid.set_cell(4, true);
        let _ = grid.set_cell(5, true);

        let result = grid.next_cells_with_change_info();
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert!(value.1);
            assert_eq!(
                value.0,
                vec![false, true, false, false, true, false, false, true, false]
            )
        }
    }
}
