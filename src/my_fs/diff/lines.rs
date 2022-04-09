use std::fmt;

use fmt::Display;

pub enum Lines {
  New { content: String },
  Deleted { content: String },
  Persistent { content: String },
}

impl Display for Lines {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::New { content } => write!(f, "\x1b[92m+ {}\x1b[0m", content),
      Self::Deleted { content } => write!(f, "\x1b[31m- {}\x1b[0m", content),
      Self::Persistent { content } => write!(f, "{}", content),
    }
  }
}
