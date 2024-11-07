use super::models::{Loan, LoanError};
use crate::library::books::models::Book;
use crate::library::users::models::User;
use std::fs::File;
use std::io::ErrorKind;

pub fn read_from_json(file_path: &str) -> Result<Vec<Loan>, LoanError> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                return Ok(Vec::new());
            } else {
                return Err(LoanError::IoError(err));
            }
        }
    };

    let loans: Vec<Loan> = serde_json::from_reader(file).map_err(LoanError::JsonError)?;

    Ok(loans)
}

pub fn save_to_json(file_path: &str, loans: &[Loan]) -> Result<(), LoanError> {
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, loans)?;
    Ok(())
}

pub fn add_loan(
    loans: &mut Vec<Loan>,
    users: &[User],
    books: &mut [Book],
    user_id: u32,
    book_id: u32,
    loan_date: String,
) -> Result<(), LoanError> {
    if !users.iter().any(|u| u.id == user_id) {
        return Err(LoanError::UserNotFound);
    }

    match books.iter_mut().find(|b| b.id == book_id) {
        Some(book) => {
            if book.is_borrowed {
                return Err(LoanError::BookNotAvailable);
            } else {
                book.is_borrowed = true;
            }
        }
        None => return Err(LoanError::BookNotFound),
    }

    if loans
        .iter()
        .any(|l| l.book_id == book_id && l.return_date.is_none())
    {
        return Err(LoanError::LoanAlreadyExists);
    }

    let loan = Loan::new(user_id, book_id, loan_date);
    loans.push(loan);
    Ok(())
}

pub fn return_loan(
    loans: &mut Vec<Loan>,
    books: &mut [Book],
    book_id: u32,
    return_date: String,
) -> Result<(), LoanError> {
    match loans
        .iter_mut()
        .find(|l| l.book_id == book_id && l.return_date.is_none())
    {
        Some(loan) => {
            loan.return_date = Some(return_date);

            if let Some(book) = books.iter_mut().find(|b| b.id == book_id) {
                book.is_borrowed = false;
            }
            Ok(())
        }
        None => Err(LoanError::LoanNotFound),
    }
}

pub fn delete_loan(loans: &mut Vec<Loan>, book_id: u32) -> Result<(), LoanError> {
    let index = loans
        .iter()
        .position(|l| l.book_id == book_id && l.return_date.is_none());
    match index {
        Some(i) => {
            loans.remove(i);
            Ok(())
        }
        None => Err(LoanError::LoanNotFound),
    }
}

pub fn get_active_loans(loans: &[Loan]) -> Vec<Loan> {
    loans
        .iter()
        .filter(|l| l.return_date.is_none())
        .cloned()
        .collect()
}

pub fn get_loans_by_user(loans: &[Loan], user_id: u32) -> Vec<Loan> {
    loans
        .iter()
        .filter(|l| l.user_id == user_id)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_add_loan_success() {
        let mut loans = Vec::new();
        let users = vec![User {
            id: 1,
            name: "Alice".to_string(),
        }];
        let mut books = vec![Book {
            id: 1,
            title: "Rust Book".to_string(),
            author: "Steve".to_string(),
            pages: 300,
            is_borrowed: false,
        }];

        let result = add_loan(
            &mut loans,
            &users,
            &mut books,
            1,
            1,
            "2023-10-01".to_string(),
        );
        assert!(result.is_ok());
        assert_eq!(loans.len(), 1);
        assert!(books[0].is_borrowed);
    }

    #[test]
    fn test_add_loan_user_not_found() {
        let mut loans = Vec::new();
        let users = vec![User {
            id: 1,
            name: "Alice".to_string(),
        }];
        let mut books = vec![Book {
            id: 1,
            title: "Rust Book".to_string(),
            author: "Steve".to_string(),
            pages: 300,
            is_borrowed: false,
        }];

        let result = add_loan(
            &mut loans,
            &users,
            &mut books,
            2,
            1,
            "2023-10-01".to_string(),
        );
        assert!(matches!(result, Err(LoanError::UserNotFound)));
        assert_eq!(loans.len(), 0);
        assert!(!books[0].is_borrowed);
    }

    #[test]
    fn test_add_loan_book_not_found() {
        let mut loans = Vec::new();
        let users = vec![User {
            id: 1,
            name: "Alice".to_string(),
        }];
        let mut books = vec![];

        let result = add_loan(
            &mut loans,
            &users,
            &mut books,
            1,
            1,
            "2023-10-01".to_string(),
        );
        assert!(matches!(result, Err(LoanError::BookNotFound)));
        assert_eq!(loans.len(), 0);
    }

    #[test]
    fn test_add_loan_book_already_borrowed() {
        let mut loans = Vec::new();
        let users = vec![User {
            id: 1,
            name: "Alice".to_string(),
        }];
        let mut books = vec![Book {
            id: 1,
            title: "Rust Book".to_string(),
            author: "Steve".to_string(),
            pages: 300,
            is_borrowed: true,
        }];

        let result = add_loan(
            &mut loans,
            &users,
            &mut books,
            1,
            1,
            "2023-10-01".to_string(),
        );
        assert!(matches!(result, Err(LoanError::BookNotAvailable)));
        assert_eq!(loans.len(), 0);
    }

    #[test]
    fn test_add_loan_already_exists() {
        let mut loans = vec![Loan {
            user_id: 1,
            book_id: 1,
            loan_date: "2023-10-01".to_string(),
            return_date: None,
        }];
        let users = vec![User {
            id: 1,
            name: "Alice".to_string(),
        }];
        let mut books = vec![Book {
            id: 1,
            title: "Rust Book".to_string(),
            author: "Steve".to_string(),
            pages: 300,
            is_borrowed: false,
        }];

        let result = add_loan(
            &mut loans,
            &users,
            &mut books,
            1,
            1,
            "2023-10-02".to_string(),
        );
        assert!(matches!(result, Err(LoanError::LoanAlreadyExists)));
        assert_eq!(loans.len(), 1);
    }

    #[test]
    fn test_return_loan_success() {
        let mut loans = vec![Loan {
            user_id: 1,
            book_id: 1,
            loan_date: "2023-10-01".to_string(),
            return_date: None,
        }];
        let mut books = vec![Book {
            id: 1,
            title: "Rust Book".to_string(),
            author: "Steve".to_string(),
            pages: 300,
            is_borrowed: true,
        }];

        let result = return_loan(&mut loans, &mut books, 1, "2023-10-10".to_string());
        assert!(result.is_ok());
        assert_eq!(loans[0].return_date, Some("2023-10-10".to_string()));
        assert!(!books[0].is_borrowed);
    }

    #[test]
    fn test_return_loan_not_found() {
        let mut loans = vec![];
        let mut books = vec![Book {
            id: 1,
            title: "Rust Book".to_string(),
            author: "Steve".to_string(),
            pages: 300,
            is_borrowed: false,
        }];

        let result = return_loan(&mut loans, &mut books, 1, "2023-10-10".to_string());
        assert!(matches!(result, Err(LoanError::LoanNotFound)));
    }

    #[test]
    fn test_delete_loan_success() {
        let mut loans = vec![Loan {
            user_id: 1,
            book_id: 1,
            loan_date: "2023-10-01".to_string(),
            return_date: None,
        }];
        let result = delete_loan(&mut loans, 1);
        assert!(result.is_ok());
        assert!(loans.is_empty());
    }

    #[test]
    fn test_delete_loan_not_found() {
        let mut loans = vec![];
        let result = delete_loan(&mut loans, 1);
        assert!(matches!(result, Err(LoanError::LoanNotFound)));
    }

    #[test]
    fn test_get_active_loans() {
        let loans = vec![
            Loan {
                user_id: 1,
                book_id: 1,
                loan_date: "2023-10-01".to_string(),
                return_date: None,
            },
            Loan {
                user_id: 2,
                book_id: 2,
                loan_date: "2023-09-01".to_string(),
                return_date: Some("2023-09-15".to_string()),
            },
        ];
        let active_loans = get_active_loans(&loans);
        assert_eq!(active_loans.len(), 1);
        assert_eq!(active_loans[0].book_id, 1);
    }

    #[test]
    fn test_get_loans_by_user() {
        let loans = vec![
            Loan {
                user_id: 1,
                book_id: 1,
                loan_date: "2023-10-01".to_string(),
                return_date: None,
            },
            Loan {
                user_id: 1,
                book_id: 2,
                loan_date: "2023-09-01".to_string(),
                return_date: Some("2023-09-15".to_string()),
            },
            Loan {
                user_id: 2,
                book_id: 3,
                loan_date: "2023-08-01".to_string(),
                return_date: Some("2023-08-15".to_string()),
            },
        ];
        let user_loans = get_loans_by_user(&loans, 1);
        assert_eq!(user_loans.len(), 2);
        assert!(user_loans.iter().all(|l| l.user_id == 1));
    }

    #[test]
    fn test_save_and_read_loans() {
        let temp_file = NamedTempFile::new().expect("Não foi possível criar arquivo temporário");
        let file_path = temp_file.path().to_str().unwrap();

        let loans = vec![
            Loan {
                user_id: 1,
                book_id: 1,
                loan_date: "2023-10-01".to_string(),
                return_date: None,
            },
            Loan {
                user_id: 2,
                book_id: 2,
                loan_date: "2023-09-01".to_string(),
                return_date: Some("2023-09-15".to_string()),
            },
        ];

        assert!(save_to_json(file_path, &loans).is_ok());

        let loaded_loans = read_from_json(file_path).expect("Falha ao ler empréstimos");
        assert_eq!(loaded_loans.len(), 2);
    }

    #[test]
    fn test_read_from_nonexistent_file() {
        let result = read_from_json("arquivo_que_nao_existe.json");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_read_from_malformed_json() {
        let mut temp_file =
            NamedTempFile::new().expect("Não foi possível criar arquivo temporário");
        writeln!(temp_file, "isto não é um JSON válido")
            .expect("Falha ao escrever no arquivo temporário");
        let file_path = temp_file.path().to_str().unwrap();

        let result = read_from_json(file_path);
        assert!(matches!(result, Err(LoanError::JsonError(_))));
    }

    #[test]
    fn test_save_to_unwritable_location() {
        let result = save_to_json("/permissao_negada/loans.json", &[]);
        assert!(matches!(result, Err(LoanError::IoError(_))));
    }
}
