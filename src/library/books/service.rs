use super::models::{Book, BookError};
use std::fs::File;
use std::io::ErrorKind;

pub fn read_from_json(file_path: &str) -> Result<Vec<Book>, BookError> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                return Ok(Vec::new());
            } else {
                return Err(BookError::IoError(err));
            }
        }
    };

    let books: Vec<Book> = serde_json::from_reader(file).map_err(BookError::JsonError)?;

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
    book_id: u32,
    title: Option<String>,
    author: Option<String>,
    pages: Option<u32>,
) -> Result<(), BookError> {
    let book = books.iter_mut().find(|book| book.id == book_id);

    match book {
        Some(book) => {
            if let Some(t) = title {
                book.title = t;
            }
            if let Some(a) = author {
                book.author = a;
            }
            if let Some(p) = pages {
                book.pages = p;
            }
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

pub fn delete_book_by_id(books: &mut Vec<Book>, book_id: u32) -> Result<(), BookError> {
    let index = books.iter().position(|book| book.id == book_id);

    match index {
        Some(i) => {
            books.remove(i);
            Ok(())
        }
        None => Err(BookError::BookNotFound),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_add_book_success() {
        let mut books = Vec::new();
        let book = Book {
            id: 1,
            title: "Rust Programming".to_string(),
            author: "Steve Klabnik".to_string(),
            pages: 550,
            is_borrowed: false,
        };
        assert!(add_book(&mut books, book.clone()).is_ok());
        assert_eq!(books.len(), 1);
    }

    #[test]
    fn test_add_book_duplicate() {
        let mut books = vec![Book {
            id: 1,
            title: "Rust Programming".to_string(),
            author: "Steve Klabnik".to_string(),
            pages: 550,
            is_borrowed: false,
        }];
        let duplicate_book = Book {
            id: 2, // ID diferente
            title: "Rust Programming".to_string(),
            author: "Steve Klabnik".to_string(),
            pages: 550,
            is_borrowed: false,
        };
        let result = add_book(&mut books, duplicate_book);
        assert!(matches!(result, Err(BookError::BookAlreadyExists)));
        assert_eq!(books.len(), 1);
    }

    #[test]
    fn test_delete_book_by_id_success() {
        let mut books = vec![
            Book {
                id: 1,
                title: "Livro Um".to_string(),
                author: "Autor A".to_string(),
                pages: 100,
                is_borrowed: false,
            },
            Book {
                id: 2,
                title: "Livro Dois".to_string(),
                author: "Autor B".to_string(),
                pages: 200,
                is_borrowed: false,
            },
        ];
        assert!(delete_book_by_id(&mut books, 1).is_ok());
        assert_eq!(books.len(), 1);
        assert_eq!(books[0].id, 2);
    }

    #[test]
    fn test_delete_book_by_id_not_found() {
        let mut books = vec![Book {
            id: 1,
            title: "Livro Um".to_string(),
            author: "Autor A".to_string(),
            pages: 100,
            is_borrowed: false,
        }];
        let result = delete_book_by_id(&mut books, 2);
        assert!(matches!(result, Err(BookError::BookNotFound)));
        assert_eq!(books.len(), 1);
    }

    #[test]
    fn test_update_book_success() {
        let mut books = vec![Book {
            id: 1,
            title: "Título Antigo".to_string(),
            author: "Autor Antigo".to_string(),
            pages: 100,
            is_borrowed: false,
        }];
        let new_title = Some("Novo Título".to_string());
        let new_author = Some("Novo Autor".to_string());
        let new_pages = Some(200);
        assert!(update_book(
            &mut books,
            1,
            new_title.clone(),
            new_author.clone(),
            new_pages
        )
        .is_ok());
        assert_eq!(books[0].title, new_title.unwrap());
        assert_eq!(books[0].author, new_author.unwrap());
        assert_eq!(books[0].pages, 200);
    }

    #[test]
    fn test_update_book_not_found() {
        let mut books = vec![Book {
            id: 1,
            title: "Título".to_string(),
            author: "Autor".to_string(),
            pages: 100,
            is_borrowed: false,
        }];
        let result = update_book(&mut books, 2, Some("Novo Título".to_string()), None, None);
        assert!(matches!(result, Err(BookError::BookNotFound)));
    }

    #[test]
    fn test_search_books_found() {
        let books = vec![
            Book {
                id: 1,
                title: "Programação em Rust".to_string(),
                author: "Steve Klabnik".to_string(),
                pages: 550,
                is_borrowed: false,
            },
            Book {
                id: 2,
                title: "O Livro".to_string(),
                author: "Autor B".to_string(),
                pages: 300,
                is_borrowed: false,
            },
        ];
        let results = search_books(&books, "Rust");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Programação em Rust");
    }

    #[test]
    fn test_search_books_not_found() {
        let books = vec![Book {
            id: 1,
            title: "Livro Um".to_string(),
            author: "Autor A".to_string(),
            pages: 100,
            is_borrowed: false,
        }];
        let results = search_books(&books, "Inexistente");
        assert!(results.is_empty());
    }

    #[test]
    fn test_save_and_read_books() {
        let temp_file = NamedTempFile::new().expect("Não foi possível criar arquivo temporário");
        let file_path = temp_file.path().to_str().unwrap();

        let books = vec![
            Book {
                id: 1,
                title: "Livro Um".to_string(),
                author: "Autor A".to_string(),
                pages: 100,
                is_borrowed: false,
            },
            Book {
                id: 2,
                title: "Livro Dois".to_string(),
                author: "Autor B".to_string(),
                pages: 200,
                is_borrowed: false,
            },
        ];
        assert!(save_to_json(file_path, &books).is_ok());

        let loaded_books = read_from_json(file_path).expect("Falha ao ler livros");
        assert_eq!(loaded_books.len(), 2);
    }

    #[test]
    fn test_read_from_nonexistent_file() {
        let result = read_from_json("arquivo_inexistente.json");
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
        assert!(matches!(result, Err(BookError::JsonError(_))));
    }

    #[test]
    fn test_save_to_unwritable_location() {
        let result = save_to_json("/permissao_negada/books.json", &[]);
        assert!(matches!(result, Err(BookError::IoError(_))));
    }
}
