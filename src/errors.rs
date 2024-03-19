use thiserror::Error;

// Result type
pub type AuthResult<T> = std::result::Result<T, AuthenticationError>;

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Token expired")]
    TokenExpired,

    #[error("Token not found")]
    TokenNotFound,

    #[error("Could not read token")]
    Io(#[from] std::io::Error),

    // Add this line to encapsulate keyring::Error
    #[error(transparent)]
    Keyring(#[from] keyring::Error),

    // JSON parsing error
    #[error("Could not parse token")]
    JsonParsingError,

    // User ID not found in token
    #[error("User ID not found")]
    UserIdNotFound,

    // Username not found in token
    #[error("Username not found")]
    UsernameNotFound,
}
