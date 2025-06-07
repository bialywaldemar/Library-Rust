// src\main.rs

mod book;
mod client;
mod library;

use book::Book;
use client::Client;
use library::Library;

fn main() -> std::io::Result<()> {
    let mut lib = Library::new();

    // Add books
    lib.add_books(vec![
        Book::new("W pustyni i w puszczy".to_string(), "Henryk Sienkiewicz".to_string()),
        Book::new("Dżuma".to_string(), "Albert Camus".to_string()),
        Book::new("Wesele".to_string(), "Stanisław Wyspiański".to_string()),
    ]);
    
    lib.add_books_from_file()?;

    // Add clients
    let client1 = Client::new("John Doe".to_string());
    let client1_id = client1.id;
    lib.add_clients(vec![client1]);
    
    lib.add_clients(vec![Client::new("Juliusz Cezar".to_string())]);
    lib.add_clients_from_file()?;
    
    // Borrow books using the safe method
    lib.borrow_book(client1_id, 1);
    lib.borrow_book(client1_id, 6);
    lib.print_client_books(client1_id);

    lib.return_book(client1_id, 6);
    lib.print_client_books(client1_id);
    Ok(())
}
