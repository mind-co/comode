use colored::{Colorize, CustomColor};

use crate::colors::ComindColors;

// enums + convenience stuff for the repl
pub enum ReplMode {
    Think,
    Search,
}

// REPL mode to prompt string
impl ReplMode {
    pub fn prompt(&self, colors: &ComindColors) -> String {
        match self {
            ReplMode::Think => format!("{}", "[think]".bold().custom_color(colors.primary())),
            ReplMode::Search => format!("{}", "[search]".bold().custom_color(colors.secondary())),
        }
    }
}
