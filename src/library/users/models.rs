use serde::{Deserialize, Serialize};
use std::fmt;
use std::io;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

impl User {
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug)]
pub enum UserError {
    IoError(io::Error),
    JsonError(serde_json::Error),
    UserNotFound,
    UserAlreadyExists,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::IoError(err) => write!(f, "IO Error: {}", err),
            UserError::JsonError(err) => write!(f, "JSON Error: {}", err),
            UserError::UserNotFound => write!(f, "User not found"),
            UserError::UserAlreadyExists => write!(f, "User already exists"),
        }
    }
}

impl std::error::Error for UserError {}

impl From<io::Error> for UserError {
    fn from(err: io::Error) -> Self {
        UserError::IoError(err)
    }
}

impl From<serde_json::Error> for UserError {
    fn from(err: serde_json::Error) -> Self {
        UserError::JsonError(err)
    }
}
