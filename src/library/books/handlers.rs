use super::service;
use crate::library::books::models::{Book, BookError};

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
    book_id: u32,
    title: Option<String>,
    author: Option<String>,
    pages: Option<u32>,
) -> Result<(), BookError> {
    service::update_book(books, book_id, title, author, pages)
}

pub(crate) fn add_book(books: &mut Vec<Book>, book: Book) -> Result<(), BookError> {
    service::add_book(books, book)
}

pub(crate) fn delete_book_by_id(books: &mut Vec<Book>, book_id: u32) -> Result<(), BookError> {
    service::delete_book_by_id(books, book_id)
}

pub(crate) fn print_books(books: &[Book]) {
    for book in books {
        println!("ID: {}", book.id);
        println!("Title: {}", book.title);
        println!("Author: {}", book.author);
        println!("Pages: {}", book.pages);
        println!("Borrowed: {}", book.is_borrowed);
        println!();
    }
}
