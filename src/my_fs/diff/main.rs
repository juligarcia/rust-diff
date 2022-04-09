use std::cmp::max;

use super::Lines;
use crate::data_structures::Grid;
use crate::File;

pub struct Diff<'a> {
  grid: Grid<u32>,
  file1: &'a File,
  file2: &'a File,
}

impl<'a> Diff<'a> {
  pub fn new(file1: &'a File, file2: &'a File) -> Self {
    let rows = file1.get_line_count();
    let cols = file2.get_line_count();

    let mut grid = Grid::new(rows + 1, cols + 1, 0_u32);

    for i in 0..rows {
      for j in 0..cols {
        if file1.compare_line(i, file2.get_line(j)) {
          grid.write(i + 1, j + 1, grid.read(i, j) + 1)
        } else {
          grid.write(
            i + 1,
            j + 1,
            max(*grid.read(i + 1, j), *grid.read(i, j + 1)),
          )
        }
      }
    }

    Self { grid, file1, file2 }
  }

  pub fn get_diff(&self) -> String {
    self.get_diff_recursive(self.file1.get_line_count(), self.file2.get_line_count())
  }

  fn get_diff_recursive(&self, i: usize, j: usize) -> String {
    if i > 0 && j > 0 && self.file1.compare_line(i - 1, self.file2.get_line(j - 1)) {
      return format!(
        "{}\n  {}",
        self.get_diff_recursive(i - 1, j - 1),
        Lines::Persistent {
          content: self.file1.get_line(i - 1).clone()
        }
      );
    }

    if j > 0 && (i == 0 || self.grid.read(i, j - 1) >= self.grid.read(i - 1, j)) {
      return format!(
        "{}\n{}",
        self.get_diff_recursive(i, j - 1),
        Lines::New {
          content: self.file2.get_line(j - 1).clone()
        }
      );
    }

    if i > 0 && (j == 0 || self.grid.read(i, j - 1) < self.grid.read(i - 1, j)) {
      return format!(
        "{}\n{}",
        self.get_diff_recursive(i - 1, j),
        Lines::Deleted {
          content: self.file1.get_line(i - 1).clone()
        }
      );
    }

    String::new()
  }
}
