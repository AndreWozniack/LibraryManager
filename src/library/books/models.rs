// books/models.rs

use serde::{Deserialize, Serialize};
use std::fmt;
use std::io;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub pages: u32,
    pub is_borrowed: bool,
}

impl Book {
    pub fn new(id: u32, title: String, author: String, pages: u32) -> Self {
        Self {
            id,
            title,
            author,
            pages,
            is_borrowed: false,
        }
    }
}

#[derive(Debug)]
pub enum BookError {
    IoError(io::Error),
    JsonError(serde_json::Error),
    BookNotFound,
    BookAlreadyExists,
}

impl fmt::Display for BookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookError::IoError(err) => write!(f, "IO Error: {}", err),
            BookError::JsonError(err) => write!(f, "JSON Error: {}", err),
            BookError::BookNotFound => write!(f, "Book not found"),
            BookError::BookAlreadyExists => write!(f, "Book already exists"),
        }
    }
}

impl std::error::Error for BookError {}

impl From<io::Error> for BookError {
    fn from(err: io::Error) -> Self {
        BookError::IoError(err)
    }
}

impl From<serde_json::Error> for BookError {
    fn from(err: serde_json::Error) -> Self {
        BookError::JsonError(err)
    }
}
