use std::fmt;

use fmt::Display;

pub enum Lines {
  AddedLine { content: String },
  RemovedLine { content: String },
  Line { content: String },
}

impl Display for Lines {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::AddedLine { content } => write!(f, "\x1b[92m+ {}\x1b[0m", content),
      Self::RemovedLine { content } => write!(f, "\x1b[31m- {}\x1b[0m", content),
      Self::Line { content } => write!(f, "{}", content),
    }
  }
}
