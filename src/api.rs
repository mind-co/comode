use crate::types::Ping;
use crate::types::Thought;
use crate::types::User;

use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;

use http::uri;
use serde_json::json;

const API_URL: &str = "https://nimbus.pfiffer.org";

/// Get user thoughts
/// get /api/user-thoughts/{user_id}
///
/// This endpoint returns the thoughts of a user.
///
/// # Parameters
/// - user_id: The user's ID
///
/// # Returns
/// - 200: The user's thoughts
/// - 401: The user is not authenticated
/// - 404: The user does not exist
/// - 500: An internal server error occurred
///
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
///
pub fn get_user_thoughts(user: &User, limit: Option<u32>, pageno: Option<u32>) -> Vec<Thought> {
    // Unpack the limit and pageno
    let limit = match limit {
        Some(limit) => limit,
        None => 100,
    };

    let pageno = match pageno {
        Some(pageno) => pageno,
        None => 0,
    };

    // Set up client
    let client = reqwest::blocking::Client::new();

    // Set up URI
    let uri = format!("{}/api/user-thoughts/{}", API_URL, user.username);

    // Set up request
    let request = client
        .get(uri)
        .header("ComindLimit", limit.to_string())
        .header("ComindPageNo", pageno.to_string())
        .header("Authorization", format!("Bearer {}", user.token));

    // Send request
    let response = request.send();

    // Unpack it
    let response = match response {
        Ok(response) => response,
        Err(e) => {
            println!("Error unpacking response: {}", e);
            return Vec::new();
        }
    };

    // Get the status code
    let status = response.status();

    // Check the status code
    if status != 200 {
        println!("get_user_thoughts status code: {}", status);
        return Vec::new();
    }

    // Get the body
    let body = match response.text() {
        Ok(body) => body,
        Err(e) => {
            println!("Body extraction get_user_thoughts: {}", e);
            return Vec::new();
        }
    };

    // Parse the response to Vec<Thought>
    let thoughts: Vec<Thought> = match serde_json::from_str(&body) {
        Ok(thoughts) => thoughts,
        Err(e) => {
            // Show the message too
            println!("Body: {}", body);
            println!("Error parsing thoughts: {}", e);
            return Vec::new();
        }
    };

    return thoughts;
}

///
/// Make a new thought
///
/// patch /api/thoughts
/// The julia code:
pub fn make_new_thought(user: &User, title: &str, body: &str) -> Result<bool, String> {
    // Set up client
    let client = reqwest::blocking::Client::new();

    // Set up URI
    let uri = format!("{}/api/thoughts", API_URL);

    // Create the body. All we need for this is title, body, and user_id.
    let body = json!({
        "title": title,
        "body": body,
        "user_id": user.user_id,
    });

    // Set up request
    let request = client
        .post(uri)
        .header("Authorization", format!("Bearer {}", user.token))
        .header("Content-Type", "application/json")
        .body(body.to_string());

    // Send request
    let response = match request.send() {
        Ok(response) => response,
        Err(e) => {
            println!("Error sending request: {}", e);
            return Err("Error sending request".to_string());
        }
    };

    // Get the status code
    let status = response.status();

    // Check the status code
    if status != 200 {
        println!("make_new_thought status code: {}", status);
        return Err("Status code not 200".to_string());
    }

    // Get the body
    // let body = match response.text() {
    //     Ok(body) => body,
    //     Err(e) => {
    //         println!("Body extraction make_new_thought: {}", e);
    //         return Err("Error extracting body".to_string());
    //     }
    // };

    // Parse the response to Thought
    // let thought: Thought = match serde_json::from_str(&body) {
    //     Ok(thought) => thought,
    //     Err(e) => {
    //         println!("Error parsing thought: {}", e);
    //         return Err("Error parsing thought".to_string());
    //     }
    // };

    // return Ok(thought);
    return Ok(true);
}

///
/// Get pings
///
/// get /api/notifications/
///
pub fn get_pings(user: &User) -> Vec<Ping> {
    // Set up client
    let client = reqwest::blocking::Client::new();

    // Set up URI
    let uri = format!("{}/api/notifications", API_URL);

    // Set up request
    let request = client
        .get(uri)
        .header("Authorization", format!("Bearer {}", user.token));

    // Send request
    let response = request.send();

    // Unpack it
    let response = match response {
        Ok(response) => response,
        Err(e) => {
            println!("Error unpacking response: {}", e);
            return Vec::new();
        }
    };

    // Get the status code
    let status = response.status();

    // Check the status code
    if status != 200 {
        println!("get_pings status code: {}", status);
        return Vec::new();
    }

    // Get the body
    let body = match response.text() {
        Ok(body) => body,
        Err(e) => {
            println!("Body extraction get_pings: {}", e);
            return Vec::new();
        }
    };

    // Parse the response to Vec<Ping>
    let pings: Vec<Ping> = match serde_json::from_str(&body) {
        Ok(pings) => pings,
        Err(e) => {
            // Show the message too
            println!("Body: {}", body);
            println!("Error parsing pings: {}", e);
            return Vec::new();
        }
    };

    return pings;
}
