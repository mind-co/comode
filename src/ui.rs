use colored::{Colorize, CustomColor};

use crate::colors::ComindColors;

// enums + convenience stuff for the repl
#[derive(PartialEq)]
pub enum UIMode {
    ThinkPublic,
    ThinkPrivate,
    ThoughtView,
    Thoughts,
    Stream,
    Pings,
    Search,
}

// REPL mode to prompt string
impl UIMode {
    pub fn prompt(&self, colors: &ComindColors) -> String {
        match self {
            UIMode::ThinkPublic => format!("{}", "[think]".bold().custom_color(colors.primary())),
            UIMode::ThinkPrivate => {
                format!("{}", "[think 🔒]".bold().custom_color(colors.primary()))
            }
            UIMode::Thoughts => format!("{}", "[thoughts]".bold().custom_color(colors.secondary())),
            UIMode::Stream => format!("{}", "[stream]".bold().custom_color(colors.tertiary())),
            UIMode::Pings => format!("{}", "[pings]".bold().custom_color(colors.tertiary())),
            UIMode::Search => format!("{}", "[search]".bold().custom_color(colors.secondary())),
            UIMode::ThoughtView => {
                format!("{}", "[thought view]".bold().custom_color(colors.primary()))
            }
        }
    }
}
///
/// A vector of modes to put on the tabs
///
pub fn modes() -> Vec<UIMode> {
    vec![
        UIMode::Thoughts,
        UIMode::Pings,
        UIMode::Search,
        UIMode::Stream,
    ]
}

///
/// Mode strings
///
pub fn mode_strings() -> Vec<String> {
    vec![
        "Thoughts".to_string(),
        "Pings".to_string(),
        "Search".to_string(),
        "Stream".to_string(),
    ]
}

/// Return the next/previous mode.
///
/// The order is as follows:
/// - ThinkPublic
/// - Thoughts
/// - Pings
pub fn next_mode(mode: UIMode) -> UIMode {
    match mode {
        UIMode::ThinkPublic => UIMode::Thoughts,
        UIMode::ThinkPrivate => UIMode::Thoughts,
        UIMode::Thoughts => UIMode::Pings,
        UIMode::Pings => UIMode::ThinkPublic,
        UIMode::Stream => UIMode::ThinkPublic,
        UIMode::Search => UIMode::ThinkPublic,
        UIMode::ThoughtView => UIMode::ThinkPublic,
    }
}

/// Return the previous mode.
///
pub fn prev_mode(mode: UIMode) -> UIMode {
    match mode {
        UIMode::ThinkPublic => UIMode::Pings,
        UIMode::ThinkPrivate => UIMode::Pings,
        UIMode::Thoughts => UIMode::ThinkPublic,
        UIMode::Pings => UIMode::Thoughts,
        UIMode::Stream => UIMode::Thoughts,
        UIMode::Search => UIMode::Thoughts,
        UIMode::ThoughtView => UIMode::Thoughts,
    }
}
