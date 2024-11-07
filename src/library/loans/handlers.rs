use super::service;
use crate::library::books::models::Book;
use crate::library::loans::models::{Loan, LoanError};
use crate::library::users::models::User;

pub(crate) fn save_loans(loans: &[Loan]) -> Result<(), LoanError> {
    service::save_to_json("loans.json", loans)
}

pub(crate) fn read_loans() -> Result<Vec<Loan>, LoanError> {
    service::read_from_json("loans.json")
}

pub(crate) fn add_loan(
    loans: &mut Vec<Loan>,
    users: &[User],
    books: &mut [Book],
    user_id: u32,
    book_id: u32,
    loan_date: String,
) -> Result<(), LoanError> {
    service::add_loan(loans, users, books, user_id, book_id, loan_date)
}

pub(crate) fn return_loan(
    loans: &mut Vec<Loan>,
    books: &mut [Book],
    book_id: u32,
    return_date: String,
) -> Result<(), LoanError> {
    service::return_loan(loans, books, book_id, return_date)
}

pub(crate) fn delete_loan(loans: &mut Vec<Loan>, book_id: u32) -> Result<(), LoanError> {
    service::delete_loan(loans, book_id)
}

pub(crate) fn print_loans(loans: &[Loan]) {
    for loan in loans {
        println!("User ID: {}", loan.user_id);
        println!("Book ID: {}", loan.book_id);
        println!("Loan Date: {}", loan.loan_date);
        match &loan.return_date {
            Some(date) => println!("Return Date: {}", date),
            None => println!("Return Date: Not returned yet"),
        }
        println!();
    }
}

pub(crate) fn get_active_loans(loans: &[Loan]) -> Vec<Loan> {
    service::get_active_loans(loans)
}

pub(crate) fn get_loans_by_user(loans: &[Loan], user_id: u32) -> Vec<Loan> {
    service::get_loans_by_user(loans, user_id)
}
