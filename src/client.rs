
// src\client.rs

use crate::book::Book;
use rand::Rng;
use std::sync::atomic::{AtomicI32, Ordering};

static CLIENT_ID_COUNTER: AtomicI32 = AtomicI32::new(1);

#[derive(Debug, Clone)]
pub struct Client {
    pub id : i32,
    pub name : String,
    pub books_curr_borrowed : Vec<i32>,
    pub books_borrowed_num : i32,
}

impl Client {
    pub fn new(name: String) -> Self {
        let id = CLIENT_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        Self {
            id,
            name,
            books_curr_borrowed: Vec::new(),
            books_borrowed_num: 0,
        }
    }

    pub fn borrow_book(&mut self, book_id: i32) {
        self.books_curr_borrowed.push(book_id);
        self.books_borrowed_num += 1;
    }

    pub fn return_book(&mut self, book_id: i32) -> bool {
        if let Some(pos) = self.books_curr_borrowed.iter().position(|&id| id == book_id) {
            self.books_curr_borrowed.remove(pos);
            self.books_borrowed_num -= 1;
            true
        } else {
            false
        }

    }

    pub fn return_random_book(&mut self) -> Option<i32>{
        if self.books_curr_borrowed.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.books_curr_borrowed.len());
        Some(self.books_curr_borrowed.remove(idx))
    }

    pub fn return_all_books(&mut self) -> Vec<i32> {
        let returned = self.books_curr_borrowed.clone();
        self.books_curr_borrowed.clear();
        self.books_borrowed_num = 0;
        returned
    }

    
}