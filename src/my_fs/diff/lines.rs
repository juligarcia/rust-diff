use std::fmt;

use fmt::Display;

/// Enum que pretende modelizar los tipos de líneas que hay en un diff
/// Nuevas, eliminadas y persistentes, todas modelizadas mediante un struct
/// que contiene su contenido como un String
pub enum Lines {
  New { content: String },
  Deleted { content: String },
  Persistent { content: String },
}

impl Display for Lines {
  /// Implementa la función format del trait Display para poder
  /// diferenciar en la consola las diferentes líneas
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::New { content } => write!(f, "\x1b[92m+ {}\x1b[0m", content),
      Self::Deleted { content } => write!(f, "\x1b[31m- {}\x1b[0m", content),
      Self::Persistent { content } => write!(f, "{}", content),
    }
  }
}
