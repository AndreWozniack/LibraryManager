use serde::{Deserialize, Serialize};
use std::fmt;
use std::io;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loan {
    pub user_id: u32,
    pub book_id: u32,
    pub loan_date: String,
    pub return_date: Option<String>,
}

impl Loan {
    pub fn new(user_id: u32, book_id: u32, loan_date: String) -> Self {
        Self {
            user_id,
            book_id,
            loan_date,
            return_date: None,
        }
    }
}

#[derive(Debug)]
pub enum LoanError {
    IoError(io::Error),
    JsonError(serde_json::Error),
    LoanNotFound,
    LoanAlreadyExists,
    BookNotAvailable,
    UserNotFound,
    BookNotFound,
}

impl fmt::Display for LoanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoanError::IoError(err) => write!(f, "IO Error: {}", err),
            LoanError::JsonError(err) => write!(f, "JSON Error: {}", err),
            LoanError::LoanNotFound => write!(f, "Loan not found"),
            LoanError::LoanAlreadyExists => write!(f, "Loan already exists"),
            LoanError::BookNotAvailable => write!(f, "Book is not available"),
            LoanError::UserNotFound => write!(f, "User not found"),
            LoanError::BookNotFound => write!(f, "Book not found"),
        }
    }
}

impl std::error::Error for LoanError {}

impl From<io::Error> for LoanError {
    fn from(err: io::Error) -> Self {
        LoanError::IoError(err)
    }
}

impl From<serde_json::Error> for LoanError {
    fn from(err: serde_json::Error) -> Self {
        LoanError::JsonError(err)
    }
}
