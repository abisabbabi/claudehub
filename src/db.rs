//! Database interaction layer for Calibre.

use rusqlite::{params, Connection};
use std::path::Path;

use crate::error::Result;
use crate::models::*;

/// Main interface for interacting with a Calibre database.
pub struct CalibreDb {
    conn: Connection,
}

impl CalibreDb {
    /// Open a Calibre database at the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the Calibre metadata.db file
    ///
    /// # Example
    ///
    /// ```no_run
    /// use calibre_db::CalibreDb;
    ///
    /// let db = CalibreDb::open("/path/to/metadata.db")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(crate::CalibreError::InvalidPath(
                format!("Database file not found: {}", path.display()),
            ));
        }

        let conn = Connection::open(path)?;
        Ok(CalibreDb { conn })
    }

    /// Get all books from the database.
    pub fn get_all_books(&self) -> Result<Vec<Book>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, sort, timestamp, pubdate, series_index, author_sort, 
                    isbn, lccn, path, flags, uuid, has_cover, last_modified 
             FROM books",
        )?;

        let books = stmt.query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                sort: row.get(2)?,
                timestamp: row.get(3)?,
                pubdate: row.get(4)?,
                series_index: row.get(5)?,
                author_sort: row.get(6)?,
                isbn: row.get(7)?,
                lccn: row.get(8)?,
                path: row.get(9)?,
                flags: row.get(10)?,
                uuid: row.get(11)?,
                has_cover: row.get::<_, i32>(12)? != 0,
                last_modified: row.get(13)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(books)
    }

    /// Get a specific book by ID.
    pub fn get_book(&self, book_id: u32) -> Result<Book> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, sort, timestamp, pubdate, series_index, author_sort, 
                    isbn, lccn, path, flags, uuid, has_cover, last_modified 
             FROM books WHERE id = ?1",
        )?;

        let book = stmt.query_row(params![book_id], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                sort: row.get(2)?,
                timestamp: row.get(3)?,
                pubdate: row.get(4)?,
                series_index: row.get(5)?,
                author_sort: row.get(6)?,
                isbn: row.get(7)?,
                lccn: row.get(8)?,
                path: row.get(9)?,
                flags: row.get(10)?,
                uuid: row.get(11)?,
                has_cover: row.get::<_, i32>(12)? != 0,
                last_modified: row.get(13)?,
            })
        })
        .map_err(|_| crate::CalibreError::BookNotFound(book_id))?;

        Ok(book)
    }

    /// Search for books by title (case-insensitive).
    pub fn search_books_by_title(&self, title: &str) -> Result<Vec<Book>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, sort, timestamp, pubdate, series_index, author_sort, 
                    isbn, lccn, path, flags, uuid, has_cover, last_modified 
             FROM books WHERE LOWER(title) LIKE LOWER(?1)",
        )?;

        let search_pattern = format!("%{}%", title);
        let books = stmt.query_map(params![search_pattern], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                sort: row.get(2)?,
                timestamp: row.get(3)?,
                pubdate: row.get(4)?,
                series_index: row.get(5)?,
                author_sort: row.get(6)?,
                isbn: row.get(7)?,
                lccn: row.get(8)?,
                path: row.get(9)?,
                flags: row.get(10)?,
                uuid: row.get(11)?,
                has_cover: row.get::<_, i32>(12)? != 0,
                last_modified: row.get(13)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(books)
    }

    /// Get all authors from the database.
    pub fn get_all_authors(&self) -> Result<Vec<Author>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, sort, link FROM authors",
        )?;

        let authors = stmt.query_map([], |row| {
            Ok(Author {
                id: row.get(0)?,
                name: row.get(1)?,
                sort: row.get(2)?,
                link: row.get(3)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(authors)
    }

    /// Get an author by ID.
    pub fn get_author(&self, author_id: u32) -> Result<Author> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, sort, link FROM authors WHERE id = ?1",
        )?;

        let author = stmt.query_row(params![author_id], |row| {
            Ok(Author {
                id: row.get(0)?,
                name: row.get(1)?,
                sort: row.get(2)?,
                link: row.get(3)?,
            })
        })
        .map_err(|_| crate::CalibreError::AuthorNotFound(author_id))?;

        Ok(author)
    }

    /// Get all books by a specific author.
    pub fn get_books_by_author(&self, author_id: u32) -> Result<Vec<Book>> {
        let mut stmt = self.conn.prepare(
            "SELECT b.id, b.title, b.sort, b.timestamp, b.pubdate, b.series_index, 
                    b.author_sort, b.isbn, b.lccn, b.path, b.flags, b.uuid, b.has_cover, b.last_modified 
             FROM books b 
             JOIN books_authors_link bal ON b.id = bal.book 
             WHERE bal.author = ?1 
             ORDER BY bal.position",
        )?;

        let books = stmt.query_map(params![author_id], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                sort: row.get(2)?,
                timestamp: row.get(3)?,
                pubdate: row.get(4)?,
                series_index: row.get(5)?,
                author_sort: row.get(6)?,
                isbn: row.get(7)?,
                lccn: row.get(8)?,
                path: row.get(9)?,
                flags: row.get(10)?,
                uuid: row.get(11)?,
                has_cover: row.get::<_, i32>(12)? != 0,
                last_modified: row.get(13)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(books)
    }

    /// Get all book series.
    pub fn get_all_series(&self) -> Result<Vec<Series>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name FROM series",
        )?;

        let series_list = stmt.query_map([], |row| {
            Ok(Series {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(series_list)
    }

    /// Get the total number of books in the database.
    pub fn get_book_count(&self) -> Result<u32> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM books")?;
        let count: u32 = stmt.query_row([], |row| row.get(0))?;
        Ok(count)
    }
}
