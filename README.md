# Calibre Database Reader

A Rust library and tools for reading and querying [Calibre](https://calibre-ebook.com/) database files.

## Overview

Calibre is a powerful ebook management system that stores book metadata in SQLite databases. This project provides a Rust interface to interact with these databases programmatically.

## Features

- 📚 Read books and metadata from Calibre databases
- 👤 Query authors and author-book relationships
- 🔍 Search for books by title
- 🏷️ Access book series and tags
- 📊 Get statistics about your Calibre library

## Getting Started

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
calibre-db = "0.1"
```

### Basic Usage

```rust
use calibre_db::CalibreDb;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the Calibre database
    let db = CalibreDb::open("/path/to/calibre/metadata.db")?;
    
    // Get all books
    let books = db.get_all_books()?;
    println!("Total books: {}", books.len());
    
    // Search for a specific book
    let results = db.search_books_by_title("rust")?;
    for book in results {
        println!("{}: {}", book.title, book.author_sort.unwrap_or_default());
    }
    
    Ok(())
}
```

## API Examples

### Get Book Count
```rust
let count = db.get_book_count()?;
println!("Books in library: {}", count);
```

### Get a Specific Book
```rust
let book = db.get_book(123)?;
println!("Title: {}", book.title);
```

### Get All Authors
```rust
let authors = db.get_all_authors()?;
for author in authors {
    println!("{}", author.name);
}
```

### Get Books by Author
```rust
let books = db.get_books_by_author(42)?;
for book in books {
    println!("{}", book.title);
}
```

### Get All Series
```rust
let series_list = db.get_all_series()?;
for series in series_list {
    println!("{}", series.name);
}
```

## Project Structure

```
.
├── src/
│   ├── lib.rs          # Main library file
│   ├── db.rs           # Database interaction layer
│   ├── models.rs       # Data models
│   └── error.rs        # Error types
├── examples/
│   └── read_calibre.rs # Example usage
├── Cargo.toml          # Project manifest
└── README.md           # This file
```

## Data Models

### Book
- `id`: Unique book identifier
- `title`: Book title
- `author_sort`: Author name(s)
- `timestamp`: When the book was added
- `pubdate`: Publication date
- `isbn`: ISBN number
- `path`: Path within Calibre library
- `has_cover`: Whether the book has a cover image
- `last_modified`: Last modification timestamp

### Author
- `id`: Unique author identifier
- `name`: Author name
- `sort`: Sort name
- `link`: Author link/URL

### Series
- `id`: Unique series identifier
- `name`: Series name

## Running Examples

Before running examples, update the database path to point to your Calibre `metadata.db` file:

```bash
# Edit the path in examples/read_calibre.rs
cargo run --example read_calibre
```

## Requirements

- Rust 1.70 or later
- Access to a Calibre `metadata.db` file

## Dependencies

- `rusqlite` - SQLite database access
- `serde` - Serialization framework
- `serde_json` - JSON support
- `thiserror` - Error handling
- `tracing` - Structured logging

## Error Handling

The library uses `Result<T>` which is `std::result::Result<T, CalibreError>`. Common errors include:

- `CalibreError::InvalidPath` - Database file not found
- `CalibreError::DatabaseError` - SQLite operation failed
- `CalibreError::BookNotFound` - Requested book doesn't exist
- `CalibreError::AuthorNotFound` - Requested author doesn't exist

## License

Apache License 2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## References

- [Calibre Project](https://calibre-ebook.com/)
- [Calibre Database Schema](https://calibre-ebook.com/)
- [Rust SQLite with rusqlite](https://github.com/rusqlite/rusqlite)
