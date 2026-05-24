//! A Rust library for interacting with Calibre database files.
//!
//! This library provides utilities to read and query Calibre's SQLite database,
//! allowing you to programmatically access book metadata and other information.

pub mod db;
pub mod error;
pub mod models;

pub use db::CalibreDb;
pub use error::{CalibreError, Result};
pub use models::*;
