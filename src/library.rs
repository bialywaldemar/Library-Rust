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
        // Find the book first
        let book = match self.books.iter_mut().find(|b| b.id == book_id) {
            Some(b) => b,
            None => return Err(format!("Book {} not found", book_id)),
        };

        // Check if already borrowed
        if book.is_borrowed {
            return Err(format!("Book {} is already borrowed", book_id));
        }

        // Then find the client
        let client = match self.clients.iter_mut().find(|c| c.id == client_id) {
            Some(c) => c,
            None => return Err(format!("Client {} not found", client_id)),
        };

        // Only update if all checks pass
        book.is_borrowed = true;
        client.borrow_book(book_id);
        Ok(())
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
        let client_idx = self.find_client_index(client_id)
            .ok_or(format!("Client {} not found", client_id))?;
        
        let book_idx = self.find_book_index(book_id)
            .ok_or(format!("Book {} not found", book_id))?;

        if !self.clients[client_idx].return_book(book_id) {
            return Err(format!("Client didn't have book {}", book_id));
        }

        self.books[book_idx].is_borrowed = false;
        println!("{} returned '{}'", 
               self.clients[client_idx].name, 
               self.books[book_idx].title);
        
        Ok(())
    }

    pub fn return_all_books(&mut self, client_id: i32) -> Result<Vec<i32>, String> {
        let client_idx = self.find_client_index(client_id)
            .ok_or(format!("Client {} not found", client_id))?;

        let returned_book_ids = {
            let client = &mut self.clients[client_idx];
            client.return_all_books()
        };

        for book_id in &returned_book_ids {
            if let Some(book) = self.get_book_mut_by_id(*book_id) {
                book.is_borrowed = false;
            }
        }

        println!(
            "{} returned {} books: {:?}",
            self.clients[client_idx].name,
            returned_book_ids.len(),
            returned_book_ids
        );

        Ok(returned_book_ids)
    }

    pub fn choose_book(&mut self) -> Option<i32> {
        // chooses random available book, better ranked books are more likely, returns its id
        let mut rng = thread_rng();   
        let available_books: Vec<&Book> = self.books
            .iter()
            .filter(|b| b.is_free())
            .collect();
        
        available_books
            .choose_weighted(&mut rng, |b| b.rating as f64)
            .ok()
            .map(|book| book.id) 
    }

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

    pub fn simulate(&mut self, days: i32, borrow_rate : f64, return_rate: f64, return_all_rate: f64) // simulates each client activity for provided number of days, in each day each client has provided probability of each of 3 activities, and also probability of not doing anything
    {
        // Validate probabilities sum to <= 1.0
        let total_rate = borrow_rate + return_rate + return_all_rate;
        if total_rate > 1.0 {
            println!("Error: Probabilities sum to {} (should be <= 1.0)", total_rate);
            return;
        }

        let mut rng = rand::thread_rng();

        for day in 1..=days {
            println!("\n=== Day {} ===", day);
            
            // Process clients in random order each day
            let mut client_indices: Vec<usize> = (0..self.clients.len()).collect();
            client_indices.shuffle(&mut rng);

            for client_idx in client_indices {
                let chance: f64 = rng.r#gen();
                let client_id = self.clients[client_idx].id;

                match chance {
                    c if c < borrow_rate => {
                        // Borrow a random book
                        if let Some(book_id) = self.choose_book() {
                            match self.borrow_book(client_id, book_id) {
                                Ok(_) => println!("{} borrowed book {}", 
                                    self.clients[client_idx].name, book_id),
                                Err(e) => println!("{} failed to borrow: {}", 
                                    self.clients[client_idx].name, e),
                            }
                        } else {
                            println!("No available books for {} to borrow", 
                                self.clients[client_idx].name);
                        }
                    },
                    c if c < borrow_rate + return_rate => {
                        // Return a random book
                        if let Some(book_id) = self.clients[client_idx].return_random_book() {
                            match self.return_book(client_id, book_id) {
                                Ok(_) => println!("{} returned book {}", 
                                    self.clients[client_idx].name, book_id),
                                Err(e) => println!("{} failed to return: {}", 
                                    self.clients[client_idx].name, e),
                            }
                        } else {
                            println!("{} has no books to return", 
                                self.clients[client_idx].name);
                        }
                    },
                    c if c < borrow_rate + return_rate + return_all_rate => {
                        // Return all books
                        match self.return_all_books(client_id) {
                            Ok(returned) => println!("{} returned all books: {:?}", 
                                self.clients[client_idx].name, returned),
                            Err(e) => println!("{} failed to return all: {}", 
                                self.clients[client_idx].name, e),
                        }
                    },
                    _ => {
                        // Do nothing
                        println!("{} did nothing today", 
                            self.clients[client_idx].name);
                    }
                }
            }

            // Print daily summary
            println!("\nDay {} summary:", day);
            println!("Available books: {}", 
                self.books.iter().filter(|b| b.is_free()).count());
            println!("Borrowed books: {}", 
                self.books.iter().filter(|b| !b.is_free()).count());
            println!("Active clients: {}", 
                self.clients.iter().filter(|c| !c.books_curr_borrowed.is_empty()).count());
        }
    }
}