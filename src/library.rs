// src\library.rs

use crate::book::Book;
use crate::client::Client;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

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
        
        println!("Added {} books from file", self.books.len());
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
            let parts: Vec<&str> = line.split(',').collect();
        }
        Ok(())
    }

    pub fn choose_book(&mut self) -> Option<&Book> {
        // chooses random available book, better ranked books are more likely
        let mut rng = thread_rng();
        let available_books: Vec<&Book> = self.books
            .iter()
            .filter(|b| b.is_free())
            .collect();
        
        available_books
            .choose_weighted(&mut rng, |b| b.rating as f64)
            .ok()
            .copied()
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
}