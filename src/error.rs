//! Error types for calibre-db operations.

use thiserror::Error;

/// Result type for calibre-db operations.
pub type Result<T> = std::result::Result<T, CalibreError>;

/// Error types that can occur when interacting with Calibre databases.
#[derive(Error, Debug)]
pub enum CalibreError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Invalid database path: {0}")]
    InvalidPath(String),

    #[error("Book not found: {0}")]
    BookNotFound(u32),

    #[error("Author not found: {0}")]
    AuthorNotFound(u32),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
