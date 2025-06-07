
// src\book.rs

use rand::Rng;
use std::sync::atomic::{AtomicI32, Ordering};

static BOOK_ID_COUNTER: AtomicI32 = AtomicI32::new(1);

#[derive(Debug, Clone)]
pub struct Book {
    pub id : i32,
    pub title : String,
    pub author: String, 
    pub is_borrowed: bool,
    pub rating : u8, // ocena ksiazki 0-100
}

impl Book {
    pub fn new(title: String, author: String) -> Self {
        let mut rng = rand::thread_rng();
        let rating = rng.gen_range(0..=100);
        let is_borrowed = false;
        let id = BOOK_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        Self {
            id,
            title,
            author,
            is_borrowed,
            rating,
        }
    }

    pub fn is_free(&self) -> bool {
        !self.is_borrowed
    }

    pub fn print(&self){
        println!("Book id: {}\nBook title: {}\nBook author: {}\nStatus: {}\nRating: {}", self.id, self.title, self.author, if self.is_borrowed {"Unavailable"} else {"Available"}, self.rating);
    }
}

