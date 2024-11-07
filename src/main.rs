mod library;

use crate::library::books::models::Book;
use crate::library::users::models::User;
use library::Library;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut library = Library::new();

    if let Err(e) = library.load_data() {
        eprintln!("Erro ao carregar os dados: {}", e);
    }

    loop {
        println!("\n===== Sistema de Gerenciamento de livros =====");
        println!("1. Adicionar Livro");
        println!("2. Adicionar Usuário");
        println!("3. Empréstimo de Livro");
        println!("4. Devolução de Livro");
        println!("5. Listar Livros");
        println!("6. Listar Usuários");
        println!("7. Listar Empréstimos Ativos");
        println!("8. Pesquisar Livros");
        println!("9. Sair");
        print!("Escolha uma opção: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => add_book(&mut library)?,
            "2" => add_user(&mut library)?,
            "3" => loan_book(&mut library)?,
            "4" => return_book(&mut library)?,
            "5" => library.list_books(),
            "6" => list_users(&library),
            "7" => library.list_active_loans(),
            "8" => search_books(&library)?,
            "9" => {
                if let Err(e) = library.save_data() {
                    eprintln!("Erro ao salvar: {}", e);
                }
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida. Tente novamente."),
        }
    }

    Ok(())
}

fn add_book(library: &mut Library) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Adicionar um livro novo ---");

    let id = prompt_for_u32("Insira o ID do Livro: ");
    let title = prompt_for_string("Insira o Título do Livro: ");
    let author = prompt_for_string("Insira o Autor do Livro: ");
    let pages = prompt_for_u32("Insira o Número de Páginas do Livro: ");

    let new_book = Book::new(id, title, author, pages);

    match library.add_book(new_book) {
        Ok(_) => println!("Livro adicionado com sucesso."),
        Err(e) => println!("Erro ao adicionar o livero: {}", e),
    }

    Ok(())
}

fn add_user(library: &mut Library) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Adicionar um novo usuário ---");

    let id = prompt_for_u32("Insira o ID do Usuário: ");
    let name = prompt_for_string("Insira o Nome do Usuário: ");

    let new_user = User::new(id, name);

    match library.add_user(new_user) {
        Ok(_) => println!("Usuário adicionado com sucesso."),
        Err(e) => println!("Erro ao adicionar o usuário: {}", e),
    }

    Ok(())
}

fn loan_book(library: &mut Library) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Empréstimo de Livro ---");

    println!("Deseja:");
    println!("1. Listar todos os livros disponíveis");
    println!("2. Pesquisar livro por nome ou autor");
    print!("Escolha uma opção: ");
    io::stdout().flush()?;

    let mut option = String::new();
    io::stdin().read_line(&mut option)?;
    let option = option.trim();

    let book_id = match option {
        "1" => {
            println!("Livros disponíveis para empréstimo:");
            for book in library.books.iter().filter(|b| !b.is_borrowed) {
                println!(
                    "ID: {}, Título: {}, Autor: {}",
                    book.id, book.title, book.author
                );
            }

            prompt_for_u32("Digite o ID do livro que deseja emprestar: ")
        }
        "2" => {
            let query = prompt_for_string("Digite o nome ou autor do livro: ");
            let results = library.search_books(&query);

            let available_books: Vec<&Book> = results.iter().filter(|b| !b.is_borrowed).collect();

            if available_books.is_empty() {
                println!("Nenhum livro disponível encontrado com esse termo.");
                return Ok(());
            } else {
                println!("Livros disponíveis encontrados:");
                for book in &available_books {
                    println!(
                        "ID: {}, Título: {}, Autor: {}",
                        book.id, book.title, book.author
                    );
                }
            }

            prompt_for_u32("Digite o ID do livro que deseja emprestar: ")
        }
        _ => {
            println!("Opção inválida.");
            return Ok(());
        }
    };

    if let Some(book) = library.books.iter().find(|b| b.id == book_id) {
        if book.is_borrowed {
            println!("Este livro já está emprestado.");
            return Ok(());
        }
    } else {
        println!("Livro não encontrado.");
        return Ok(());
    }

    println!("Usuários cadastrados:");
    for user in &library.users {
        println!("ID: {}, Nome: {}", user.id, user.name);
    }

    let user_id = prompt_for_u32("Digite o ID do usuário: ");

    if !library.users.iter().any(|u| u.id == user_id) {
        println!("Usuário não encontrado.");
        return Ok(());
    }

    let loan_date = prompt_for_string("Digite a data do empréstimo (YYYY-MM-DD): ");

    match library.loan_book(user_id, book_id, loan_date) {
        Ok(_) => println!("Livro emprestado com sucesso."),
        Err(e) => println!("Erro ao emprestar livro: {}", e),
    }

    Ok(())
}
fn return_book(library: &mut Library) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Devolução de Livros ---");

    let book_id = prompt_for_u32("Insira o ID do Livro: ");
    let return_date = prompt_for_string("Insira a Data de Devolução (YYYY-MM-DD): ");

    match library.return_book(book_id, return_date) {
        Ok(_) => println!("Livro devolvido com sucesso."),
        Err(e) => println!("Erro ao devolver o livro: {}", e),
    }

    Ok(())
}

fn list_users(library: &Library) {
    println!("\n--- Lista de usuários ---");
    if library.users.is_empty() {
        println!("Nenhum usuário cadastrado.");
    } else {
        for user in &library.users {
            println!("ID: {}, Nome: {}", user.id, user.name);
        }
    }
}

fn search_books(library: &Library) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Pesquisar Livros ---");

    let query = prompt_for_string("Digite o nome ou autor do livro: ");
    let results = library.search_books(&query);

    if results.is_empty() {
        println!("Nenhum livro encontrado com esse termo.");
    } else {
        println!("Livros encontrados:");
        for book in results {
            println!(
                "ID: {}, Titulo: {}, Autor: {}, Paginas: {}",
                book.id, book.title, book.author, book.pages
            );
        }
    }

    Ok(())
}

fn prompt_for_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn prompt_for_u32(prompt: &str) -> u32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Entrada inválida. Tente novamente."),
        }
    }
}
