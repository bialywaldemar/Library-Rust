// src\main.rs

mod book;
mod client;
mod library;

use book::Book;
use library::Library;

fn main() -> std::io::Result<()> {
    let mut lib = Library::new();

    let mut book1 = Book::new("W pustyni i w puszczy".to_string(), "Henryk Sienkiewicz".to_string());
    let mut book2 = Book::new("Dżuma".to_string(), "Albert Camus".to_string());
    let mut book3 = Book::new("Wesele".to_string(), "Stanisław Wyspiański".to_string());

    let books1 = vec![book1, book2, book3];
    lib.add_books(books1);
    
    lib.add_books_from_file()?;
    
    lib.print();
    Ok(())
}
