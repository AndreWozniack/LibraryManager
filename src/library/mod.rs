pub(crate) mod books;
mod loans;
pub(crate) mod users;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use books::handlers as book_handlers;
use loans::handlers as loan_handlers;
use users::handlers as user_handlers;

use books::models::{Book, BookError};
use loans::models::{Loan, LoanError};
use users::models::{User, UserError};

pub struct Library {
    pub(crate) books: Vec<Book>,
    pub(crate) users: Vec<User>,
    loans: Vec<Loan>,
}
#[warn(dead_code)]
impl Library {
    pub fn new() -> Self {
        Self {
            books: Vec::new(),
            users: Vec::new(),
            loans: Vec::new(),
        }
    }
    fn ensure_file_exists(file_path: &str) -> std::io::Result<()> {
        if !Path::new(file_path).exists() {
            let mut file = File::create(file_path)?;
            file.write_all(b"[]")?;
        }
        Ok(())
    }

    pub fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Self::ensure_file_exists("books.json")?;
        Self::ensure_file_exists("users.json")?;
        Self::ensure_file_exists("loans.json")?;

        self.books = book_handlers::read_books()?;
        self.users = user_handlers::read_users()?;
        self.loans = loan_handlers::read_loans()?;
        Ok(())
    }

    pub fn save_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        book_handlers::save_books(&self.books)?;
        user_handlers::save_users(&self.users)?;
        loan_handlers::save_loans(&self.loans)?;
        Ok(())
    }

    pub fn add_book(&mut self, book: Book) -> Result<(), BookError> {
        book_handlers::add_book(&mut self.books, book)
    }

    pub fn remove_book(&mut self, book_id: u32) -> Result<(), BookError> {
        book_handlers::delete_book_by_id(&mut self.books, book_id)
    }

    pub fn search_books(&self, query: &str) -> Vec<Book> {
        book_handlers::search_books(&self.books, query)
    }

    pub fn list_books(&self) {
        book_handlers::print_books(&self.books);
    }

    pub fn add_user(&mut self, user: User) -> Result<(), UserError> {
        user_handlers::add_user(&mut self.users, user)
    }

    pub fn loan_book(
        &mut self,
        user_id: u32,
        book_id: u32,
        loan_date: String,
    ) -> Result<(), LoanError> {
        loan_handlers::add_loan(
            &mut self.loans,
            &self.users,
            &mut self.books,
            user_id,
            book_id,
            loan_date,
        )
    }

    pub fn return_book(&mut self, book_id: u32, return_date: String) -> Result<(), LoanError> {
        loan_handlers::return_loan(&mut self.loans, &mut self.books, book_id, return_date)
    }

    pub fn list_active_loans(&self) {
        let active_loans = loan_handlers::get_active_loans(&self.loans);
        loan_handlers::print_loans(&active_loans);
    }
}
