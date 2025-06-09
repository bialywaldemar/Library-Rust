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

    // Simulate 30 days with:
    // - 40% chance to borrow
    // - 30% chance to return one
    // - 10% chance to return all
    // - 20% chance to do nothing
    lib.simulate(30, 0.2, 0.2, 0.1);
        
    // Borrow books using the safe method
    // if let Err(e) = lib.borrow_book(client1_id, 1) {
    //     eprintln!("Borrow error: {}", e);
    // }
    // if let Err(e) = lib.borrow_book(client1_id, 6) {
    //     eprintln!("Borrow error: {}", e);
    // }
    // lib.print_client_books(client1_id);

    // if let Err(e) = lib.return_book(client1_id, 6) {
    //     eprintln!("Return error: {}", e);
    // }
    // lib.print_client_books(client1_id);

    // // Store the chosen book ID outside the if-let scope
    // let mut book_chosen = None;
    // if let Some(book_id) = lib.choose_book() {
    //     book_chosen = Some(book_id);
    //     if let Err(e) = lib.borrow_book(client1_id, book_id) {
    //         eprintln!("Borrow error: {}", e);
    //     }
    // } else {
    //     eprintln!("No available books to choose");
    // }

    // // Additional borrows with error handling
    // if let Err(e) = lib.borrow_book(client1_id, 9) {
    //     eprintln!("Borrow error: {}", e);
    // }
    // if let Err(e) = lib.borrow_book(client1_id, 2) {
    //     eprintln!("Borrow error: {}", e);
    // }

    // lib.print_client_books(client1_id);
    
    // if let Err(e) = lib.return_all_books(client1_id) {
    //     eprintln!("Return all error: {}", e);
    // }
    // lib.print_client_books(client1_id);
    
    Ok(())
}
