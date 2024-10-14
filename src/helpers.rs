use diesel::prelude::*;
use diesel::SqliteConnection;
use rocket::local::blocking::Client;
use rocket::{self, routes};
use crate::routes::{get_todos, add_todo, delete_todo, update_todo, complete_todo};
use std::fs;
use std::path::Path;
use diesel::sql_query;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_test_connection() -> DbPool{
    let database_url = "test.sqlite".to_string();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url.clone());
    info!("Establishing database connection with {}", database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Set PRAGMA busy_timeout to retry if the database is locked
    let mut connection = pool.get().expect("Failed to get connection from pool.");
    diesel::sql_query("PRAGMA busy_timeout = 3000;")  // Retry for 3 seconds
        .execute(&mut connection)
        .expect("Failed to set PRAGMA busy_timeout");

    pool
}

pub fn setup_rocket() -> Client {
    let manager = ConnectionManager::<SqliteConnection>::new("test.sqlite");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let rocket = rocket::build()
        .manage(pool)
        .mount("/", routes![get_todos, add_todo, delete_todo, update_todo, complete_todo]);
    Client::tracked(rocket).expect("valid rocket instance")
}

pub fn run_seed_script(pool: &DbPool) -> Result<(), diesel::result::Error> {
    info!("Running seed script");

    // Get a connection from the pool
    let mut connection = pool.get().expect("Failed to get connection from pool.");

    let sql = include_str!("../tests/seed.sql");
    for statement in sql.split(';') {
        let trimmed = statement.trim();
        if !trimmed.is_empty() {
            sql_query(trimmed)
                .execute(&mut connection)?;  // Now using pooled connection
        }
    }
    Ok(())
}

pub fn cleanup_database() {
    info!("Cleaning up database");

    let path = "test.sqlite";
    if Path::new(path).exists() {
        if let Err(e) = fs::remove_file(path) {
            error!("Failed to remove test.sqlite: {:?}", e);
        } else {
            info!("Successfully removed test.sqlite");
        }
    } else {
        info!("No test.sqlite file found to remove.");
    }
}