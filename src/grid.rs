use std::fmt::Display;

#[derive(Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub enum GridError {
    OutOfBounds,
}

#[derive(Clone)]
pub struct Grid {
    pub cells: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![false; width]; height],
            width,
            height,
        }
    }

    pub fn get_all_coords(&self) -> Vec<Coord> {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| Coord { x, y }))
            .collect()
    }

    pub fn is_valid_coord(&self, coord: &Coord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub fn cell_state(&self, coord: &Coord) -> Result<bool, GridError> {
        if !self.is_valid_coord(coord) {
            return Err(GridError::OutOfBounds);
        }

        Ok(self.cells[coord.y][coord.x])
    }

    pub fn set_cell_state(&mut self, coord: &Coord, state: bool) -> Result<(), GridError> {
        if !self.is_valid_coord(&coord) {
            return Err(GridError::OutOfBounds);
        }

        self.cells[coord.y][coord.x] = state;
        Ok(())
    }

    pub fn next_cell_state(&self, coord: &Coord) -> bool {
        let current_state = self.cell_state(coord).unwrap_or(false);
        let living_neighbors_count = self.count_living_neighbors(coord);

        matches!(
            (current_state, living_neighbors_count),
            (true, 2) | (true, 3) | (false, 3)
        )
    }

    fn count_living_neighbors(&self, coord: &Coord) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = coord.x as i32 + dx;
                let ny = coord.y as i32 + dy;

                if nx >= 0 && ny >= 0 {
                    let neighbor_coord = Coord {
                        x: nx as usize,
                        y: ny as usize,
                    };

                    if let Ok(is_alive) = self.cell_state(&neighbor_coord) {
                        if is_alive {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for &cell in row {
                write!(f, " {} ", get_symbol(cell))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn get_symbol(value: bool) -> &'static str {
    if value { "■" } else { "•" }
}
