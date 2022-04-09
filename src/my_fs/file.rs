use std::{fmt, fs};

use fmt::Display;

use super::Diff;

/// Se representa un archivo como su contenido separado en líneas
pub struct File {
  /// Donde cada línea se define como un String
  lines: Vec<String>,
}

impl File {

  /// Se cuenta con un constructor a partir de la ruta donde se encuentra el archivo
  pub fn new(filename: &str) -> Self {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");

    let split = contents.split('\n');

    let lines: Vec<String> = split.map(str::to_string).collect();

    Self { lines }
  }

  /// Método que devuelve la cantidad de líneas
  pub fn get_line_count(&self) -> usize {
    self.lines.len()
  }

  /// Método que devuelve una struct Diff que simboliza el diff entre archivos
  pub fn diff(&self, external_file: &Self) -> String {
    Diff::new(self, external_file).get_diff()
  }

  /// Método que compara la línea indexada por index con la recibida en line
  pub fn compare_line(&self, index: usize, line: &str) -> bool {
    self.lines[index] == *line
  }

  /// Devuelve una referencia a una línea del archivo indexada en index
  pub fn get_line(&self, index: usize) -> &String {
    &self.lines[index]
  }
}

/// Se implementa el trait Display
impl Display for File {
  /// Formatea al struct File
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.lines.join("\n"))
  }
}
