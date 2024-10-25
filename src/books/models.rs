use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
}

pub enum BookError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    BookNotFound,
    BookAlreadyExists,
}
