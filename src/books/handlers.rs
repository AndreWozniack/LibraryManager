use super::service;
use crate::books::models::{Book, BookError};

pub(crate) fn save_books(books: &[Book]) -> Result<(), BookError> {
    service::save_to_json("books.json", books)
}

pub(crate) fn read_books() -> Result<Vec<Book>, BookError> {
    service::read_from_json("books.json")
}

pub(crate) fn search_books(books: &[Book], query: &str) -> Vec<Book> {
    service::search_books(books, query)
}

pub(crate) fn update_book(
    books: &mut Vec<Book>,
    title: &str,
    author: &str,
    pages: u32,
) -> Result<(), BookError> {
    service::update_book(books, title, author, pages)
}

pub(crate) fn add_book(books: &mut Vec<Book>, book: Book) -> Result<(), BookError> {
    service::add_book(books, book)
}

pub(crate) fn delete_book(
    books: &mut Vec<Book>,
    title: &str,
    author: &str,
) -> Result<(), BookError> {
    service::delete_book(books, title, author)
}

pub(crate) fn print_books(books: &[Book]) {
    for book in books {
        println!("Title: {}", book.title);
        println!("Author: {}", book.author);
        println!("Pages: {}", book.pages);
        println!();
    }
}
