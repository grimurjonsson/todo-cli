use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Navigate,  // Default: browse, mark, move, delete
    Edit,      // Text input for new/editing items
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Navigate => write!(f, "NAVIGATE"),
            Mode::Edit => write!(f, "INSERT"),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Navigate
    }
}
