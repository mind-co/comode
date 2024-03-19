use colored::Colorize;

use crate::colors;

// Functions for pretty display
pub fn co_say(message: &str, colors: &colors::ComindColors) {
    println!(
        "{} {}",
        "{co}".bold().custom_color(colors.primary()),
        message
    );
}
