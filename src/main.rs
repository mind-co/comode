use colored::{Colorize, CustomColor};
use jwt::{Claims, Header, Token};
use keyring::Entry;
use serde_json::Value;
use std::io::{stdin, stdout, Write};
// use std::io;
// use term_size;

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
mod repl;
use repl::ReplMode;

struct User {
    token: String,
    user_id: String,
    username: String,
}

// Create a dummy user
fn create_blank_user() -> User {
    User {
        token: String::new(),
        user_id: String::new(),
        username: String::new(),
    }
}

struct LoginResponse {
    token: String,
    message: String,
    success: bool,
}

impl User {
    fn create_from_entry(entry: &Entry) -> AuthResult<User> {
        // Load the JWT from the keyring
        let token = entry.get_password();

        // If we can't get the token, return a TokenNotFound error
        let token = match token {
            Ok(token) => token,
            Err(_) => return Err(AuthenticationError::TokenNotFound),
        };

        // The token is a string, so we need to parse it into JSON
        let token: Value = match serde_json::from_str(&token) {
            Ok(token) => token,
            Err(_) => return Err(AuthenticationError::JsonParsingError),
        };

        // Extract the "token" field from the JSON
        let token = match token["token"].as_str() {
            Some(token) => token.to_string(),
            None => return Err(AuthenticationError::TokenNotFound),
        };

        // Now the token is a string, we need to decode the JWT
        let unverified: Token<Header, Claims, _> = Token::parse_unverified(&token).unwrap();
        let issuer = unverified.claims();

        // Get expuration from issuer.registered
        let exp = match issuer.registered.expiration {
            Some(exp) => exp,
            None => return Err(AuthenticationError::TokenExpired),
        };

        // Get the current time
        let now: u64 = chrono::Utc::now().timestamp().try_into().unwrap();

        // If the token is expired, return a TokenExpired error
        if now > exp {
            return Err(AuthenticationError::TokenExpired);
        }

        // Extract the user_id from the private field
        let user_id = match issuer.private["user_id"].as_str() {
            Some(private) => private.to_string(),
            None => return Err(AuthenticationError::UserIdNotFound),
        };

        // Extract the username
        let username = match issuer.private["username"].as_str() {
            Some(private) => private.to_string(),
            None => return Err(AuthenticationError::UsernameNotFound),
        };

        // Return a blank user, DEBUG
        return Ok(User {
            token: token,
            user_id: user_id,
            username: username,
        });
    }
}

fn main() {
    // Get the terminal width
    // let width = match term_size::dimensions() {
    //     Some((w, _)) => w,
    //     None => 80,
    // };

    // Default colors
    let colors = ComindColors::default();

    // Print the header
    // println!("{:|<1$}", "", width);

    // Print blank line
    println!();

    // Print the welcome.
    co_say(
        &format!(
            "{} {}{}{}{}",
            "hey, welcome to",
            "{".bold().custom_color(colors.primary()),
            "co".bold().custom_color(colors.secondary()),
            "mind".bold().custom_color(colors.tertiary()),
            "}".bold().custom_color(colors.primary())
        ),
        &colors,
    );

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
            return;
        }
    };

    // Log in
    let user = match login() {
        Some(user) => user,
        None => {
            println!("Login failed");
            return;
        }
    };

    // Notify the user that we're logged in
    println!("{} logged in", "✓".bold().green());

    // Print the user's username
    co_say(&format!("welcome back {}", user.username), &colors);

    // Set the REPL mode
    let mut repl_mode = ReplMode::Think;

    // Tell the user
    co_say(
        &format!(
            "you're in the comind repl, type {} to see commands.",
            "help".bold()
        ),
        &colors,
    );

    // Explain Think mode
    co_say(
        &format!(
            "you're in {} mode. type a new thought to send to comind",
            repl_mode.prompt(&colors)
        ),
        &colors,
    );

    // Enter REPL
    while true {
        // Print the prompt
        print!("{} ", repl_mode.prompt(&colors));

        let mut input = String::new();
        stdout().flush().unwrap();

        // Read the input
        stdin().read_line(&mut input).unwrap();

        // Match to literals
        match input.as_str() {
            "logout\n" => {
                // Log out
                login::logout(&colors);
                println!("{} logged out", "✓".bold().green());
                return;
            }
            "exit\n" => {
                return;
            }
            "token\n" => {
                println!("token: {}", user.token);
                continue;
            }
            "user_id\n" => {
                println!("user_id: {}", user.user_id);
                continue;
            }
            "username\n" => {
                println!("username: {}", user.username);
                continue;
            }
            "search\n" => {
                repl_mode = ReplMode::Search;
                continue;
            }
            "help\n" => {
                println!("commands: logout, exit, token, user_id, username, help");
                continue;
            }
            _ => {}
        }
    }
}
