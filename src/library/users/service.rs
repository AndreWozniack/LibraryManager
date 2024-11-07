use super::models::{User, UserError};
use std::fs::File;
use std::io::ErrorKind;

pub fn read_from_json(file_path: &str) -> Result<Vec<User>, UserError> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                return Ok(Vec::new());
            } else {
                return Err(UserError::IoError(err));
            }
        }
    };

    let users: Vec<User> = serde_json::from_reader(file).map_err(UserError::JsonError)?;

    Ok(users)
}

pub fn save_to_json(file_path: &str, users: &[User]) -> Result<(), UserError> {
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, users)?;
    Ok(())
}

pub fn search_users(users: &[User], query: &str) -> Vec<User> {
    users
        .iter()
        .filter(|user| user.name.contains(query))
        .cloned()
        .collect()
}

pub fn add_user(users: &mut Vec<User>, user: User) -> Result<(), UserError> {
    if users.iter().any(|u| u.id == user.id) {
        Err(UserError::UserAlreadyExists)
    } else {
        users.push(user);
        Ok(())
    }
}

pub fn delete_user(users: &mut Vec<User>, id: u32) -> Result<(), UserError> {
    let index = users.iter().position(|u| u.id == id);
    match index {
        Some(i) => {
            users.remove(i);
            Ok(())
        }
        None => Err(UserError::UserNotFound),
    }
}

pub fn update_user(users: &mut Vec<User>, id: u32, name: String) -> Result<(), UserError> {
    let user = users.iter_mut().find(|u| u.id == id);
    match user {
        Some(u) => {
            u.name = name;
            Ok(())
        }
        None => Err(UserError::UserNotFound),
    }
}

/// Test Cases
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_add_user_success() {
        let mut users = Vec::new();
        let user = User {
            id: 1,
            name: "Alice".to_string(),
        };
        assert!(add_user(&mut users, user.clone()).is_ok());
        assert_eq!(users.len(), 1);
    }

    #[test]
    fn test_add_user_duplicate_id() {
        let mut users = vec![User {
            id: 1,
            name: "Alice".to_string(),
        }];
        let duplicate_user = User {
            id: 1,
            name: "Bob".to_string(),
        };
        let result = add_user(&mut users, duplicate_user);
        assert!(matches!(result, Err(UserError::UserAlreadyExists)));
        assert_eq!(users.len(), 1);
    }

    #[test]
    fn test_delete_user_success() {
        let mut users = vec![
            User {
                id: 1,
                name: "Alice".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
            },
        ];
        assert!(delete_user(&mut users, 1).is_ok());
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].id, 2);
    }

    #[test]
    fn test_delete_user_not_found() {
        let mut users = vec![User {
            id: 1,
            name: "Alice".to_string(),
        }];
        let result = delete_user(&mut users, 2);
        assert!(matches!(result, Err(UserError::UserNotFound)));
        assert_eq!(users.len(), 1);
    }

    #[test]
    fn test_update_user_success() {
        let mut users = vec![User {
            id: 1,
            name: "Alice".to_string(),
        }];
        let new_name = "Alice Smith".to_string();
        assert!(update_user(&mut users, 1, new_name.clone()).is_ok());
        assert_eq!(users[0].name, new_name);
    }

    #[test]
    fn test_update_user_not_found() {
        let mut users = vec![User {
            id: 1,
            name: "Alice".to_string(),
        }];
        let result = update_user(&mut users, 2, "Bob".to_string());
        assert!(matches!(result, Err(UserError::UserNotFound)));
    }

    #[test]
    fn test_search_users_found() {
        let users = vec![
            User {
                id: 1,
                name: "Alice".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
            },
            User {
                id: 3,
                name: "Charlie".to_string(),
            },
        ];
        let results = search_users(&users, "Bob");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Bob");
    }

    #[test]
    fn test_search_users_not_found() {
        let users = vec![
            User {
                id: 1,
                name: "Alice".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
            },
        ];
        let results = search_users(&users, "Charlie");
        assert!(results.is_empty());
    }

    #[test]
    fn test_save_and_read_users() {
        let temp_file = NamedTempFile::new().expect("Não foi possível criar arquivo temporário");
        let file_path = temp_file.path().to_str().unwrap();

        let users = vec![
            User {
                id: 1,
                name: "Alice".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
            },
        ];

        assert!(save_to_json(file_path, &users).is_ok());

        let loaded_users = read_from_json(file_path).expect("Falha ao ler usuários");
        assert_eq!(loaded_users.len(), 2);
        // assert_eq!(loaded_users, users);
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
        assert!(matches!(result, Err(UserError::JsonError(_))));
    }

    #[test]
    fn test_save_to_unwritable_location() {
        let result = save_to_json("/permissao_negada/users.json", &[]);
        assert!(matches!(result, Err(UserError::IoError(_))));
    }
}
