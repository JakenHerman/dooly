#[macro_use] extern crate rocket;

mod db;
mod schema; 

use db::{establish_connection, TodoItem, NewTodoItem};
use rocket::serde::{json::Json};
use rocket::http::Status;
use crate::schema::todos;
use diesel::prelude::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_todos, add_todo, update_todo, delete_todo])
}

// Fetch all to-do items from the database
#[get("/todos")]
fn get_todos() -> Result<Json<Vec<TodoItem>>, (Status, &'static str)> {
    let mut connection = establish_connection();
    let todos: Vec<TodoItem> = todos::table.load(&mut connection).map_err(|_| (Status::InternalServerError, "Failed to fetch todos"))?;
    Ok(Json(todos))
}

// Add a new to-do item to the database
#[post("/todos", format = "json", data = "<new_todo>")]
fn add_todo(new_todo: Json<NewTodoItem>) -> Result<&'static str, (Status, &'static str)> {
    let mut connection = establish_connection();
    let new_todo = NewTodoItem { title: &new_todo.title, completed: new_todo.completed };
    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(&mut connection)
        .map_err(|_| (Status::InternalServerError, "Failed to add todo"))?;
    Ok("Todo added successfully!")
}

// Update an existing to-do item
#[put("/todos/<id>", format = "json", data = "<updated_todo>")]
fn update_todo(id: i32, updated_todo: Json<TodoItem>) -> Result<&'static str, (Status, &'static str)> {
    let mut connection = establish_connection();
    diesel::update(todos::table.find(id))
        .set((todos::title.eq(&updated_todo.title), todos::completed.eq(updated_todo.completed)))
        .execute(&mut connection)
        .map_err(|_| (Status::InternalServerError, "Failed to update todo"))?;
    Ok("Todo updated successfully!")
}

// Delete a to-do item
#[delete("/todos/<id>")]
fn delete_todo(id: i32) -> Result<&'static str, (Status, &'static str)> {
    let mut connection = establish_connection();
    diesel::delete(todos::table.find(id))
        .execute(&mut connection)
        .map_err(|_| (Status::InternalServerError, "Failed to delete todo"))?;
    Ok("Todo deleted successfully!")
}