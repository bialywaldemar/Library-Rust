// src\library.rs

use crate::book::Book;
use crate::client::Client;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Library{
    pub books : Vec<Book>,
    pub clients: Vec<Client>,
}

impl Library {
    pub fn new() -> Self {
        Self {
            books: Vec::new(),
            clients: Vec::new(),
        }
    }

    pub fn add_books(&mut self, books: Vec<Book>) {
        self.books.extend(books);
    }

    pub fn add_books_from_file(&mut self) -> io::Result<()> {
        println!("Current directory: {:?}", env::current_dir()?);
        let path = Path::new("./books.txt");
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            
            if parts.len() >= 2 {
                let title = parts[0].trim().to_string();
                let author = parts[1].trim().to_string();
                self.books.push(Book::new(title, author));
            } else {
                eprintln!("Skipping invalid line: {}", line);
            }
        }
        
        Ok(())
    }

    pub fn add_clients(&mut self, clients: Vec<Client>) {
        self.clients.extend(clients);
    }

    pub fn add_clients_from_file(&mut self) -> io::Result<()> {
        let path = Path::new("./clients.txt");
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let name = line.trim().to_string();
            self.clients.push(Client::new(name));
        }
        Ok(())
    }

    pub fn borrow_book(&mut self, client_id: i32, book_id: i32) -> Result<(), String> {
        // Find the client (using immutable borrow first)
        let client = match self.clients.iter_mut().find(|c| c.id == client_id) {
            Some(c) => c,
            None => return Err(format!("Client {} not found", client_id)),
        };

        // Find and update the book
        match self.books.iter_mut().find(|b| b.id == book_id) {
            Some(book) => {
                if book.is_borrowed {
                    return Err(format!("Book {} is already borrowed", book_id));
                }
                book.is_borrowed = true;
                client.borrow_book(book_id);
                Ok(())
            }
            None => Err(format!("Book {} not found", book_id)),
        }
    }

    fn get_book_mut_by_id(&mut self, book_id: i32) -> Option<&mut Book> {
        self.books.iter_mut().find(|b| b.id == book_id)
    }

    fn find_client_index(&self, client_id: i32) -> Option<usize> {
        self.clients.iter().position(|c| c.id == client_id)
    }

    fn find_book_index(&self, book_id: i32) -> Option<usize> {
        self.books.iter().position(|b| b.id == book_id)
    }

    pub fn return_book(&mut self, client_id: i32, book_id: i32) -> Result<(), String> {
        // First find all indices using immutable borrows
        let client_idx = self.find_client_index(client_id)
            .ok_or(format!("Client {} not found", client_id))?;
        
        let book_idx = self.find_book_index(book_id)
            .ok_or(format!("Book {} not found", book_id))?;

        // Now perform mutable operations using indices
        if !self.clients[client_idx].return_book(book_id) {
            return Err(format!("Client didn't have book {}", book_id));
        }

        self.books[book_idx].is_borrowed = false;
        println!("{} returned '{}'", 
               self.clients[client_idx].name, 
               self.books[book_idx].title);
        
        Ok(())
    }

    // pub fn choose_book(&mut self) -> Option<&Book> {
    //     // chooses random available book, better ranked books are more likely
    //     let mut rng = thread_rng();   
    //     let available_books: Vec<&Book> = self.books
    //         .iter()
    //         .filter(|b| b.is_free())
    //         .collect();
        
    //     available_books
    //         .choose_weighted(&mut rng, |b| b.rating as f64)
    //         .ok()
    //         .copied()
    // }

    pub fn print(&self) {
        println!("BOOKS:");
        for book in self.books.iter() {
            println!("{}, {}, {}, {}", book.title, book.author, if book.is_free() {"Available"} else {"Unavailable"}, book.rating);
        }
        println!("CLIENTS:");
        for client in self.clients.iter() {
            println!("{}", client.name);
        }
    }

    pub fn print_client_books(&self, client_id: i32) {
        if let Some(client) = self.clients.iter().find(|c| c.id == client_id) {
            println!("Books borrowed by {} ({}):", client.name, client.id);
            for book_id in &client.books_curr_borrowed {
                if let Some(book) = self.books.iter().find(|b| b.id == *book_id) {
                    println!("- ID: {}, Title: '{}', Author: '{}'", 
                        book.id, book.title, book.author);
                }
            }
        } else {
            println!("Client with ID {} not found", client_id);
        }
    }
}