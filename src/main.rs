use colored::CustomColor;
use jwt::{Claims, Header, Token};
use keyring::Entry;
use serde_json::Value;
use std::{
    cmp::{max, min},
    io::{self, stdin, stdout, Write},
};
use types::Thought;

// terminal shit
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use termimad; // for markdown printing

// Error types
mod errors;
use errors::{AuthResult, AuthenticationError};

// Login
mod login;
use login::login;

// display types
mod display;
use display::co_say;

// color types
mod colors;
use colors::ComindColors;

// repl shit
mod ui;
use ui::{mode_strings, next_mode, prev_mode, UIMode};

// Types
mod types;

// api
mod api;
use api::get_user_thoughts;

use crate::utils::iso8601_to_datetime;

// Utils
mod utils;

fn main() -> io::Result<()> {
    // Set up the terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Default colors
    let colors = ComindColors::default();

    // // Print blank line
    // println!();

    // Print the welcome.
    // co_say(
    //     &format!(
    //         "{}",
    //         "hey, welcome to comind",
    //         // "{".bold().custom_color(colors.primary()),
    //         // "co".bold().custom_color(colors.secondary()),
    //         // "mind".bold().custom_color(colors.tertiary()),
    //         // "}".bold().custom_color(colors.primary())
    //     ),
    //     &colors,
    // );

    //     "co: {} {}{}{}{}",
    //     "hey, welcome to",
    //     "{".bold().custom_color(colors.primary()),
    //     "co".bold().custom_color(colors.secondary()),
    //     "mind".bold().custom_color(colors.tertiary()),
    //     "}".bold().custom_color(colors.primary())
    // );

    // Load the token from the keyring
    let token_entry = match Entry::new("comind", "token") {
        Ok(entry) => entry,
        Err(e) => {
            println!("Keyring load error: {:?}", e);
            return Err(io::Error::new(io::ErrorKind::Other, "Keyring load error"));
        }
    };

    // Log in
    let user = match login() {
        Some(user) => user,
        None => {
            println!("Login failed");
            return Ok(());
        }
    };

    // Skip prameter
    let skip = true;

    if !skip {
        let mut should_quit = false;
        while !should_quit {
            terminal.draw(|f| {
                start_screen(f, &user.username);
            })?;
            should_quit = handle_events()?.should_quit;
        }
    }

    // Set the REPL mode
    let mut repl_mode = UIMode::ThinkPublic;

    // Tell the user
    // co_say(
    //     &format!(
    //         "you're in the comind repl, type {} to see commands.",
    //         "help".bold()
    //     ),
    //     &colors,
    // );

    // Explain Think mode
    // co_say(
    //     &format!(
    //         "you're in {} mode. type a new thought to send to comind",
    //         repl_mode.prompt(&colors)
    //     ),
    //     &colors,
    // );

    // Get user thoughts
    let user_thoughts = get_user_thoughts(&user, None, None);
    // co_say(
    //     &format!("we loaded {} of your thoughts", user_thoughts.len()),
    //     &colors,
    // );

    // Enter REPL
    let mut should_quit = false;
    let mut ui_state = UIState {
        mode: UIMode::Thoughts,
        tab: 0,
        thoughts: user_thoughts,
        cursor_position: 0,
        selected_thought: None,
    };
    while !should_quit {
        let thought_count = ui_state.thoughts.len();
        terminal.draw(|f| {
            ui(f, &ui_state);
        })?;
        match handle_events() {
            Ok(result) => {
                if result.should_quit {
                    should_quit = true;
                }
                if result.next_tab {
                    ui_state = UIState {
                        mode: next_mode(ui_state.mode),
                        tab: (ui_state.tab + 1) % 2,
                        thoughts: ui_state.thoughts,
                        cursor_position: ui_state.cursor_position,
                        selected_thought: ui_state.selected_thought,
                    };
                }
                if result.prev_tab {
                    ui_state = UIState {
                        mode: prev_mode(ui_state.mode),
                        tab: (ui_state.tab + 1) % 2,
                        thoughts: ui_state.thoughts,
                        cursor_position: ui_state.cursor_position,
                        selected_thought: ui_state.selected_thought,
                    };
                }
                if result.up {
                    ui_state = UIState {
                        mode: ui_state.mode,
                        tab: ui_state.tab,
                        thoughts: ui_state.thoughts,
                        cursor_position: if ui_state.cursor_position == 0 {
                            thought_count - 1
                        } else {
                            ui_state.cursor_position - 1
                        },
                        selected_thought: ui_state.selected_thought,
                    };
                }
                if result.down {
                    ui_state = UIState {
                        mode: ui_state.mode,
                        tab: ui_state.tab,
                        thoughts: ui_state.thoughts,
                        cursor_position: if ui_state.cursor_position == thought_count - 1 {
                            0
                        } else {
                            ui_state.cursor_position + 1
                        },
                        selected_thought: ui_state.selected_thought,
                    };
                }
                if result.selected {
                    let thought = ui_state.thoughts[ui_state.cursor_position].clone();
                    ui_state = UIState {
                        mode: ui_state.mode,
                        tab: ui_state.tab,
                        thoughts: ui_state.thoughts,
                        cursor_position: ui_state.cursor_position,
                        selected_thought: Some(thought),
                    };
                }
                if result.escape {
                    ui_state = UIState {
                        mode: ui_state.mode,
                        tab: ui_state.tab,
                        thoughts: ui_state.thoughts,
                        cursor_position: ui_state.cursor_position,
                        selected_thought: None,
                    };
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                should_quit = true;
            }
        }
    }

    // CLose up shop
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    return Ok(());
}

// Event handling return struct
struct EventResult {
    should_quit: bool,
    next_tab: bool,
    prev_tab: bool,
    up: bool,
    down: bool,
    selected: bool,
    escape: bool,
}

// Default event result, no events
fn default_event_result() -> EventResult {
    return EventResult {
        should_quit: false,
        next_tab: false,
        prev_tab: false,
        up: false,
        down: false,
        selected: false,
        escape: false,
    };
}

fn handle_events() -> io::Result<EventResult> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            // Show the key
            // println!("{:?}", key);

            // Character matching
            match key.code {
                KeyCode::Char('c') => {
                    if key
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL)
                    {
                        return Ok(EventResult {
                            should_quit: true,
                            ..default_event_result()
                        });
                    }
                }
                KeyCode::Up => {
                    return Ok(EventResult {
                        up: true,
                        ..default_event_result()
                    })
                }
                KeyCode::Down => {
                    return Ok(EventResult {
                        down: true,
                        ..default_event_result()
                    })
                }
                KeyCode::Tab => {
                    return Ok(EventResult {
                        next_tab: true,
                        ..default_event_result()
                    })
                }
                KeyCode::BackTab => {
                    return Ok(EventResult {
                        prev_tab: true,
                        ..default_event_result()
                    })
                }
                KeyCode::Char('q') => {
                    return Ok(EventResult {
                        should_quit: true,
                        ..default_event_result()
                    })
                }
                KeyCode::Enter => {
                    return Ok(EventResult {
                        selected: true,
                        ..default_event_result()
                    })
                }
                KeyCode::Esc => {
                    return Ok(EventResult {
                        escape: true,
                        ..default_event_result()
                    })
                }
                _ => {}
            }
        }
    }

    return Ok(default_event_result());
}

// Intro screen, "welcome to comind" in center
fn start_screen(frame: &mut Frame, username: &str) {
    let size = frame.size();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(size);

    // let welcome = "hey, welcome to {comind}\n\nit's good to see you again";
    let welcome = format!(
        "hey, welcome to comind\nit's good to see you again, {}",
        username
    );
    frame.render_widget(
        Paragraph::new(welcome)
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .padding(ratatui::widgets::Padding::new(0, 0, size.height / 2, 0))
                    .border_type(BorderType::Rounded),
            )
            .centered()
            .alignment(Alignment::Center),
        layout[0],
    );
}

// UI State
struct UIState {
    mode: UIMode,
    tab: usize,
    thoughts: Vec<Thought>,
    cursor_position: usize,
    selected_thought: Option<Thought>,
}

// Main screen, with thoughts and pings
fn ui(frame: &mut Frame, ui_state: &UIState) {
    // Outer layout
    let outer_layout =
        Layout::new(Direction::Vertical, [Constraint::Percentage(100)]).split(frame.size());

    // First, check if there's a selected thought
    if let Some(thought) = &ui_state.selected_thought {
        // Thought text
        let thought_text = format!("{}\n\n", thought.body.as_str());

        // Render the thought
        let thought_text = Line::from(vec![Span::styled(
            thought_text.as_str(),
            Style::default().fg(Color::White).bg(Color::Black),
        )]);

        // Get Thought title
        let title = match &thought.title {
            Some(title) => title,
            None => " ∘ ",
        };

        let thought = Paragraph::new(thought_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" {} ", title)),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left);

        frame.render_widget(thought, outer_layout[0]);

        return;
    }

    // Tabs first
    let modes = mode_strings();
    let tabs = Tabs::new(modes)
        .block(Block::default().borders(Borders::TOP).title("comind"))
        .style(Style::default().white())
        .highlight_style(Style::default().yellow())
        .select(ui_state.tab)
        .divider(symbols::DOT);

    frame.render_widget(tabs, outer_layout[0]);

    // Display thoughts if tab 0
    if ui_state.mode == UIMode::Thoughts {
        // Inner layout
        let inner_layout =
            Layout::new(Direction::Horizontal, [Constraint::Fill(1)]).split(outer_layout[0]);

        // Make the left side
        // let left = Block::default()
        //     .title("Thoughts")
        //     .borders(Borders::ALL)
        //     .padding(Padding::uniform(4))
        //     .border_type(BorderType::Rounded)
        //     .style(Style::default().fg(Color::White).bg(Color::Red));
        // frame.render_widget(left, inner_layout[0]);

        // Thought strings
        let thoughts = ui_state
            .thoughts
            .iter()
            .enumerate()
            .map(|(i, thought)| {
                let col = if i % 2 == 0 {
                    Color::Black
                } else {
                    Color::Rgb(1, 1, 1)
                };

                // Two forms of lines to display. If the thought has a newline in it, display only
                // the title. Otherwise, display only the body. Include username and relative time
                // in both.
                //
                // Start with a span for the username and relative time, since those are common to
                // both forms.
                let cursor = if ui_state.cursor_position == i {
                    "👉 "
                } else {
                    "   "
                };
                let username_span = Span::styled(
                    format!("{}[{}] ", cursor, thought.username),
                    Style::default().fg(Color::White).bg(col).bold(),
                );
                let body_or_title = if thought.body.len() < 80 && !thought.body.contains("\n") {
                    Span::styled(
                        thought.body.as_str(),
                        Style::default().fg(Color::White).bg(col),
                    )
                } else {
                    match thought.title {
                        Some(ref title) => Span::styled(
                            title.as_str(),
                            Style::default().fg(Color::White).bg(col).underlined(),
                        ),
                        None => Span::styled(
                            thought.body.as_str(),
                            Style::default().fg(Color::White).bg(col),
                        ),
                    }
                };

                let body_line = Line::from(vec![username_span, body_or_title]);
                // let text = Text::from(vec![body_line, username_date_line]);

                ListItem::new(body_line).style(Style::default().fg(Color::White).bg(col))
            })
            .collect::<Vec<ListItem>>();

        let thoughts = List::new(thoughts).block(Block::default().padding(Padding::uniform(2)));

        frame.render_widget(thoughts, inner_layout[0]);
    }

    // Thoughts
    // let thoughts = vec![
    //     ListItem::new("This is a thought"),
    //     ListItem::new("This is another thought"),
    // ];

    // let thoughts = List::new(thoughts)
    //     .block(Block::default().borders(Borders::ALL).title("Thoughts"))
    //     .style(Style::default().fg(Color::White).bg(Color::Black))
    //     .highlight_style(Style::default().fg(Color::Black).bg(Color::White));

    // frame.render_widget(thoughts, main_layout[0]);
}
