use std::{fmt, fs};

use fmt::Display;

use super::Diff;

pub struct File {
  lines: Vec<String>,
}

impl File {
  pub fn new(filename: &str) -> Self {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");

    let split = contents.split('\n');

    let lines: Vec<String> = split.map(str::to_string).collect();

    Self { lines }
  }

  pub fn get_line_count(&self) -> usize {
    self.lines.len()
  }

  pub fn diff(&self, external_file: &Self) -> String {
    Diff::new(self, external_file).get_diff()
  }

  pub fn compare_line(&self, index: usize, line: &String) -> bool {
    self.lines[index] == *line
  }

  pub fn get_line(&self, index: usize) -> &String {
    &self.lines[index]
  }
}

impl Display for File {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", "")
  }
}
