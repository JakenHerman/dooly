use diesel::prelude::*;
use diesel::SqliteConnection;
use rocket::local::blocking::Client;
use rocket::{self, routes};
use crate::routes::{get_todos, add_todo, delete_todo};
use std::fs;
use diesel::sql_query;
use diesel::r2d2::{self, ConnectionManager};

pub fn establish_connection() -> SqliteConnection {
    let database_url = "test.sqlite".to_string();
    SqliteConnection::establish(&database_url).expect("Failed to create database connection")
}

pub fn setup_rocket() -> Client {
    let manager = ConnectionManager::<SqliteConnection>::new("test.sqlite");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let rocket = rocket::build()
        .manage(pool)
        .mount("/", routes![get_todos, add_todo, delete_todo]);
    Client::tracked(rocket).expect("valid rocket instance")
}

pub fn run_seed_script(connection: &mut SqliteConnection) {
    let sql = include_str!("../tests/seed.sql");
    for statement in sql.split(';') {
        let trimmed = statement.trim();
        if !trimmed.is_empty() {
            sql_query(trimmed)
                .execute(connection)
                .expect("Failed to run seed script");
        }
    }
}
pub fn cleanup_database() {
    let _ = fs::remove_file("test.sqlite");
}