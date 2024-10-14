use diesel::prelude::*;
use diesel::SqliteConnection;
use rocket::serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{self, Pool, ConnectionManager};

use crate::schema::todos;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = todos)]
pub struct NewTodoItem<'a> {
    pub title: &'a str,
    pub completed: bool,
}

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
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