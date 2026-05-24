//! Data models for Calibre database entities.

use serde::{Deserialize, Serialize};

/// Represents a book in the Calibre database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub sort: Option<String>,
    pub timestamp: Option<String>,
    pub pubdate: Option<String>,
    pub series_index: Option<f64>,
    pub author_sort: Option<String>,
    pub isbn: Option<String>,
    pub lccn: Option<String>,
    pub path: String,
    pub flags: u32,
    pub uuid: Option<String>,
    pub has_cover: bool,
    pub last_modified: String,
}

/// Represents an author in the Calibre database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: u32,
    pub name: String,
    pub sort: Option<String>,
    pub link: Option<String>,
}

/// Represents a book series in the Calibre database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub id: u32,
    pub name: String,
}

/// Represents a book tag/category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: u32,
    pub name: String,
}

/// Represents the relationship between a book and its authors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookAuthor {
    pub book_id: u32,
    pub author_id: u32,
    pub position: u32,
}
