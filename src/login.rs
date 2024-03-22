use keyring::Entry;
use std::{
    collections::HashMap,
    io::{self, Write},
};

// For talking to the server https
use reqwest;

use crate::types::{self, User};
use crate::{colors::ComindColors, display::co_say};

pub fn login() -> Option<User> {
    let entry = Entry::new("comind", "token").unwrap();
    let mut username = String::new();
    let mut password = String::new();

    // Check if the user is already logged in.
    // If they are, return the user.
    // If this fails, try to clear the token from the keyring
    // and continue with the login process.
    let token = entry.get_password();
    if let Ok(token) = token {
        let user = crate::types::User::create_from_entry(&entry);
        return Some(user.unwrap());
    } else {
        if let Err(e) = token {
            println!("Error: {}", e);
            entry.delete_password().unwrap();
        }
    }

    // TODO #1 make username and password input secure
    print!("Enter your username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();

    print!("Enter your password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();

    // Send the username and password to the server at
    // https://nimbus.pfiffer.org/api/login
    // and get a JWT back. Store the JWT in the keyring.
    // The server requires a username or email with a password.
    // TODO #2 make server URL configurable
    let mut request_body = HashMap::new();
    request_body.insert("username", username.trim());
    request_body.insert("password", password.trim());

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://nimbus.pfiffer.org/api/login/")
        .json(&request_body)
        .send();

    // Unpack the result
    let res = match res {
        Ok(res) => res,
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    };

    // If the server returns a JWT, store it in the keyring
    let token = res.text().unwrap();
    entry.set_password(&token).unwrap();

    // Create a user from the entry
    let user = crate::types::User::create_from_entry(&entry);
    return Some(user.unwrap());
}

// Logout function
pub fn logout(colors: &ComindColors) {
    let entry = Entry::new("comind", "token").unwrap();
    entry.delete_password().unwrap();

    // Print a message
    co_say("i've logged you out, go out and crush stuff", colors);
}
