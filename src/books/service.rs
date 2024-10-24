use super::models::{Book, BookError};
use std::fs::File;
pub fn read_from_json(file_path: &str) -> Result<Vec<Book>, BookError> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(BookError::IoError(err)),
    };

    let books: Vec<Book> = match serde_json::from_reader(file) {
        Ok(books) => books,
        Err(err) => return Err(BookError::JsonError(err)),
    };

    Ok(books)
}

pub fn save_to_json(file_path: &str, books: &[Book]) -> Result<(), BookError> {
    let file = match File::create(file_path) {
        Ok(file) => file,
        Err(err) => return Err(BookError::IoError(err)),
    };

    match serde_json::to_writer_pretty(file, books) {
        Ok(_) => Ok(()),
        Err(err) => Err(BookError::JsonError(err)),
    }
}

pub fn search_books(books: &[Book], query: &str) -> Vec<Book> {
    books
        .iter()
        .filter(|book| book.title.contains(query) || book.author.contains(query))
        .cloned()
        .collect()
}

pub fn update_book(
    books: &mut Vec<Book>,
    title: &str,
    author: &str,
    pages: u32,
) -> Result<(), BookError> {
    let book = books
        .iter_mut()
        .find(|book| book.title == title && book.author == author);

    match book {
        Some(book) => {
            book.pages = pages;
            Ok(())
        }
        None => Err(BookError::BookNotFound),
    }
}

pub fn add_book(books: &mut Vec<Book>, book: Book) -> Result<(), BookError> {
    if books
        .iter()
        .any(|b| b.title == book.title && b.author == book.author)
    {
        return Err(BookError::BookAlreadyExists);
    }

    books.push(book);
    Ok(())
}

pub fn delete_book(books: &mut Vec<Book>, title: &str, author: &str) -> Result<(), BookError> {
    let index = books
        .iter()
        .position(|book| book.title == title && book.author == author);

    match index {
        Some(index) => {
            books.remove(index);
            Ok(())
        }
        None => Err(BookError::BookNotFound),
    }
}
