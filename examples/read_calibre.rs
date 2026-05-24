//! Example: Reading books from a Calibre database

use calibre_db::CalibreDb;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the Calibre database
    // Note: Replace with your actual path to metadata.db
    let db = CalibreDb::open("path/to/calibre/metadata.db")?;

    // Get and print all books
    println!("=== All Books ===");
    let books = db.get_all_books()?;
    println!("Total books: {}\n", books.len());

    for book in books.iter().take(5) {
        println!("Title: {}", book.title);
        if let Some(author) = &book.author_sort {
            println!("Author: {}", author);
        }
        if let Some(isbn) = &book.isbn {
            println!("ISBN: {}", isbn);
        }
        println!();
    }

    // Search for a specific book
    println!("=== Search Example ===");
    let search_results = db.search_books_by_title("rust")?;
    println!("Found {} books matching 'rust'\n", search_results.len());

    // Get all authors
    println!("=== All Authors ===");
    let authors = db.get_all_authors()?;
    println!("Total authors: {}\n", authors.len());

    for author in authors.iter().take(5) {
        println!("Author: {}", author.name);
    }

    Ok(())
}
