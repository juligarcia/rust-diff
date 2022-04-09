use std::fmt;

use fmt::Display;

pub struct Grid<T: Clone + Display> {
  grid: Vec<Vec<T>>,
}

impl<T: Clone + Display> Display for Grid<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for row in self.grid.iter() {
      for col in row {
        write!(f, " {} ", col)?;
      }

      writeln!(f)?;
    }

    return write!(f, "");
  }
}

impl<T: Clone + Display> Grid<T> {
  pub fn new(rows: usize, cols: usize, init: T) -> Self {
    let mut grid = Vec::new();

    for i in 0..rows {
      grid.push(Vec::new());

      for _ in 0..cols {
        grid[i].push(init.clone());
      }
    }

    Self { grid }
  }

  pub fn write(&mut self, x: usize, y: usize, new_value: T) {
    self.grid[x][y] = new_value;
  }

  pub fn read(&self, x: usize, y: usize) -> &T {
    &self.grid[x][y]
  }
}
