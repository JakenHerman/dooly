use diesel::prelude::*;
use diesel::SqliteConnection;
use rocket::serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{self, Pool, ConnectionManager};

use crate::schema::todos;

#[derive(Queryable, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = todos)]
pub struct NewTodoItem<'a> {
    pub title: &'a str,
    pub completed: bool,
}

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url.clone());
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
