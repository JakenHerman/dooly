use rocket::http::Status;
use rocket::State;
use rocket::serde::{json::Json};
use crate::db::{TodoItem, NewTodoItem};
use crate::schema::todos;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// Fetch all to-do items from the database
#[get("/todos")]
pub fn get_todos(pool: &State<DbPool>) -> Result<Json<Vec<TodoItem>>, (Status, &'static str)> {
    let mut connection = pool.get().map_err(|_| (Status::InternalServerError, "Failed to get connection from pool"))?;
    
    let todos: Vec<TodoItem> = todos::table.load(&mut connection)
        .map_err(|_| (Status::InternalServerError, "Failed to fetch todos"))?;
    
    Ok(Json(todos))
}

// Add a new to-do item to the database
#[post("/todos", format = "json", data = "<new_todo>")]
pub fn add_todo(pool: &State<DbPool>, new_todo: Json<NewTodoItem>) -> Result<&'static str, (Status, &'static str)> {
    let mut connection = pool.get().map_err(|_| (Status::InternalServerError, "Failed to get connection from pool"))?;
    let new_todo = NewTodoItem { title: &new_todo.title, completed: new_todo.completed };
    
    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(&mut connection)
        .map_err(|_| (Status::InternalServerError, "Failed to add todo"))?;
    
    Ok("Todo added successfully!")
}

// Delete a to-do item
#[delete("/todos/<id>")]
pub fn delete_todo(pool: &State<DbPool>, id: i32) -> Result<&'static str, (Status, &'static str)> {
    let mut connection = pool.get().map_err(|_| (Status::InternalServerError, "Failed to get connection from pool"))?;
    
    diesel::delete(todos::table.find(id))
        .execute(&mut connection)
        .map_err(|_| (Status::InternalServerError, "Failed to delete todo"))?;
    
    Ok("Todo deleted successfully!")
}
