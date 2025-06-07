// src\main.rs

mod book;
mod client;
mod library;

use book::Book;
use library::Library;

fn main() -> std::io::Result<()> {
    let mut lib = Library::new();
    
    lib.add_books_from_file()?;
    
    lib.print();
    Ok(())
}
