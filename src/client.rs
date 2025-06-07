// src\client.rs

use crate::book::Book;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Client {
    pub id : i32,
    pub name : String,
    pub books_curr_borrowed : Vec<Book>,
    pub books_borrowed_num : i32,
}

impl Client {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
            books_curr_borrowed: Vec::new(),
            books_borrowed_num: 0,
        }
    }

    pub fn borrow_book(&mut self, book: &mut Book) {
        if book.is_borrowed {
            println!("Nie można wypożyczyć, książka {} już wypożyczona.", book.title);
            return;
        }

        book.is_borrowed = true;
        self.books_curr_borrowed.push(book.clone());
        self.books_borrowed_num += 1;
        println!("Wypożyczono książkę {}.", book.title);
        }

    pub fn return_book(&mut self, book_id: i32) {
        if let Some(index) = self.books_curr_borrowed.iter().position(|b| b.id == book_id) {
            self.books_curr_borrowed[index].is_borrowed = false;
            let book = self.books_curr_borrowed.remove(index);
            println!("Zwrócono książkę {}.", book.title);
        } else {
            println!("Książka nieznaleziona w wypożyczeniach.");
        }

    }

    pub fn return_random_book(&mut self) {
        let mut rng = rand::thread_rng();
        let n = self.books_curr_borrowed.len();
        if n == 0 {
            return
        }
        let index = rng.gen_range(0..n);
        self.books_curr_borrowed[index].is_borrowed = false;
        self.books_curr_borrowed.remove(index);
    }

    pub fn return_all_books(&mut self) {
        for book in self.books_curr_borrowed.iter_mut() {
            book.is_borrowed = false;
        }
        self.books_curr_borrowed.clear();
        println!("Zwrócono wszystkie książki.");
    }

    pub fn list_borrowed_books(&self) {
        for book in self.books_curr_borrowed.iter() {
            println!("{}, {}", book.title, book.author);
        }
    }
}