#[macro_use]
extern crate diesel;

use helpers::password;
use model::user::*;
use repository::users::UsersRepository;

mod helpers;
mod model;
mod repository;
mod schema;

fn main() {
    let repository = UsersRepository::init();

    let users = match repository.get_users() {
        Ok(users) => users,
        Err(e) => panic!("Error: {:?}", e),
    };

    println!("Users: {:?}", users);

    let password = String::from("trashbox123");
    let new_user = User::new(
        String::from("polcrazpolcraz@gmail.com"),
        password.clone(),
        None,
    );

    match repository.insert_user(new_user.clone()) {
        Ok(()) => println!("Insertion successful: {:?}", new_user),
        Err(e) => println!("Error: {:?}", e),
    };

    let users2 = match repository.get_users() {
        Ok(users) => users,
        Err(e) => panic!("Error: {:?}", e),
    };

    println!("Users: {:?}", users2);

    let stored_user = match repository.get_user_by_id(new_user.id.clone()) {
        Ok(res) => match res {
            Some(user) => user,
            None => panic!("Error: user not found"),
        },
        Err(e) => panic!("Error: {:?}", e),
    };

    println!("Stored user: {:?}", stored_user);

    match password::verify(password, stored_user.salt, stored_user.encrypted_password) {
        true => println!("Password verification: {:?}", true),
        false => panic!("Password verification: {:?}", false),
    };

    match repository.delete_user(stored_user.id) {
        Ok(()) => println!("Deletion successful: {:?}", new_user),
        Err(e) => println!("Error: {:?}", e),
    };

    let users3 = match repository.get_users() {
        Ok(users) => users,
        Err(e) => panic!("Error: {:?}", e),
    };

    println!("Users: {:?}", users3);
}
