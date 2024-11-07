use super::service;
use crate::library::users::models::{User, UserError};

pub(crate) fn save_users(users: &[User]) -> Result<(), UserError> {
    service::save_to_json("users.json", users)
}

pub(crate) fn read_users() -> Result<Vec<User>, UserError> {
    service::read_from_json("users.json")
}

pub(crate) fn search_users(users: &[User], query: &str) -> Vec<User> {
    service::search_users(users, query)
}

pub(crate) fn add_user(users: &mut Vec<User>, user: User) -> Result<(), UserError> {
    service::add_user(users, user)
}

pub(crate) fn delete_user(users: &mut Vec<User>, id: u32) -> Result<(), UserError> {
    service::delete_user(users, id)
}

pub(crate) fn update_user(users: &mut Vec<User>, id: u32, name: String) -> Result<(), UserError> {
    service::update_user(users, id, name)
}

pub(crate) fn print_users(users: &[User]) {
    for user in users {
        println!("ID: {}", user.id);
        println!("Name: {}", user.name);
        println!();
    }
}
