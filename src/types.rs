use crate::errors;
use crate::errors::AuthResult;
use crate::errors::AuthenticationError;
use colored::{Colorize, CustomColor};
use jwt::{Claims, Header, Token};
use keyring::Entry;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::io::{stdin, stdout, Write};
use uuid::Uuid;

/// # Example
///
/// ```json
///  [{
///           "title": "Thought ID refresh issue in dual submission problem",
///            "body": "i've narrowed the dual submission problem to thought ids not being refreshed when a thought is submitted",
///    "date_created": "2024-01-05T13:18:21.26",
///    "date_updated": "2024-03-14T16:36:44.764",
///        "revision": 0,
///              "id": "139f78be-c4e8-5f17-8d60-06162fbed802",
///          "public": true,
///       "synthetic": false,
///          "origin": "app",
///         "user_id": "6b129b9f-b958-4cf0-a6ad-3108d221177a",
///        "username": "cameron",
///           "to_id": "139f78be-c4e8-5f17-8d60-06162fbed802",
///         "n_links": 1,
///        "numlinks": 1
/// }]
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Thought {
    pub title: Option<String>,
    pub body: String,
    pub date_created: String,
    pub date_updated: String,
    pub revision: i32,
    pub id: String,
    pub public: bool,
    pub synthetic: bool,
    pub origin: String,
    pub user_id: String,
    pub username: String,
    pub to_id: Option<String>,
    pub n_links: Option<i32>,
    pub numlinks: Option<i32>,
}
impl Thought {
    pub(crate) fn clone(&self) -> Thought {
        Thought {
            title: self.title.clone(),
            body: self.body.clone(),
            date_created: self.date_created.clone(),
            date_updated: self.date_updated.clone(),
            revision: self.revision,
            id: self.id.clone(),
            public: self.public,
            synthetic: self.synthetic,
            origin: self.origin.clone(),
            user_id: self.user_id.clone(),
            username: self.username.clone(),
            to_id: self.to_id.clone(),
            n_links: self.n_links.clone(),
            numlinks: self.numlinks.clone(),
        }
    }
}

///
/// Load a thought from a JSON string
///
pub fn load_thought(json: &str) -> Result<Thought> {
    match serde_json::from_str(json) {
        Ok(thought) => Ok(thought),
        Err(e) => {
            println!("Error deserializing thought: {}", e);
            println!("{}", json);
            Err(e)
        }
    }
}

///
/// Pings
///
/// Notifications are sent to the user when a thought is linked to their thought
///
#[derive(Deserialize, Serialize, Debug)]
pub struct Ping {
    pub id: u32,
    pub r#type: String,
    pub message: String,
    pub created_at: String,
    pub read_status: bool,
    pub user_thought_id: String,
    pub linking_thought_id: String,
    pub linking_user_id: String,
    pub user_id: String,
}

///
/// Load a ping from a JSON string
///
/// # Example
///
/// ```json
/// {
///    "id": 1,
///   "type": "link",
/// "message": "thought linked",
/// "created_at": "2024-01-05T13:18:21.26",
/// "read_status": false,
/// "user_thought_id": "139f78be-c4e8-5f17-8d60-06162fbed802",
/// "linking_thought_id": "139f78be-c4e8-5f17-8d60-06162fbed802",
/// "linking_user_id": "6b129b9f-b958-4cf0-a6ad-3108d221177a",
/// "user_id": "6b129b9f-b958-4cf0-a6ad-3108d221177a"
/// }
/// ```
///
pub fn load_ping(json: &str) -> Result<Ping> {
    match serde_json::from_str(json) {
        Ok(ping) => Ok(ping),
        Err(e) => {
            println!("Error deserializing ping: {}", e);
            println!("{}", json);
            Err(e)
        }
    }
}

pub(crate) struct User {
    pub token: String,
    pub user_id: String,
    pub username: String,
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
    pub fn create_from_entry(entry: &Entry) -> AuthResult<User> {
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
