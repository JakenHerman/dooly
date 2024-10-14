use diesel::prelude::*;
use diesel::SqliteConnection;
use rocket::serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;

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

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
