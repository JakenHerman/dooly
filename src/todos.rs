use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use crate::db::DbPool;
use crate::schema::todos;
use diesel::prelude::*;
use log::info;
use chrono::NaiveDate;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub due_date: Option<NaiveDate>,
    pub completed: bool,
    pub user_id: i32,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = todos)]
pub struct NewTodoItem<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub priority: Option<i32>,
    pub due_date: Option<NaiveDate>,
    pub completed: bool,
    pub user_id: i32,  // Associate the new todo with a user
}

// Fetch all to-do items from the database
#[get("/todos")]
pub fn get_todos(pool: &State<DbPool>) -> Result<Json<Vec<TodoItem>>, (Status, &'static str)> {
    info!("Fetching all to-do items");
    let mut connection = pool.get().map_err(|_| (Status::InternalServerError, "Failed to get connection from pool"))?;
    
    let todos: Vec<TodoItem> = todos::table.load(&mut connection)
        .map_err(|_| (Status::InternalServerError, "Failed to fetch todos"))?;

    info!("Fetched {} to-do items", todos.len());
    Ok(Json(todos))
}

// Add a new to-do item to the database
#[post("/todos", format = "json", data = "<new_todo>")]
pub fn add_todo(pool: &State<DbPool>, new_todo: Json<NewTodoItem>) -> Result<&'static str, (Status, &'static str)> {
    if new_todo.title.trim().is_empty() {
        return Err((Status::BadRequest, "Title cannot be empty"));
    }

    if new_todo.completed {
        return Err((Status::BadRequest, "New todo item cannot be marked as completed"));
    }

    info!("Adding a new to-do item: {:?}", new_todo);
    let mut connection = pool.get().map_err(|_| (Status::InternalServerError, "Failed to get connection from pool"))?;
    let new_todo = NewTodoItem { title: &new_todo.title, completed: new_todo.completed, user_id: new_todo.user_id, description: new_todo.description, priority: new_todo.priority, due_date: new_todo.due_date };
    
    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(&mut connection)
        .map_err(|_| (Status::InternalServerError, "Failed to add todo"))?;
    
    Ok("Todo added successfully!")
}

// Delete a to-do item
#[delete("/todos/<id>")]
pub fn delete_todo(pool: &State<DbPool>, id: i32) -> Result<&'static str, (Status, &'static str)> {
    info!("Deleting to-do item with id: {}", id);
    let mut connection = pool.get().map_err(|_| (Status::InternalServerError, "Failed to get connection from pool"))?;
    
    diesel::delete(todos::table.find(id))
        .execute(&mut connection)
        .map_err(|_| (Status::InternalServerError, "Failed to delete todo"))?;
    
    Ok("Todo deleted successfully!")
}

#[put("/todos/<id>", format = "json", data = "<updated_todo>")]
pub fn update_todo(
    pool: &State<DbPool>, 
    id: i32, 
    updated_todo: Json<NewTodoItem>
) -> Result<&'static str, (Status, &'static str)> {
    if updated_todo.title.trim().is_empty() {
        return Err((Status::BadRequest, "Title cannot be empty"));
    }

    info!("Updating to-do item with id: {}", id);
    info!("Updated to-do item: {:?}", updated_todo);
    let mut connection = pool.get().map_err(|err| {
        error!("Failed to get connection from pool: {:?}", err);
        (Status::InternalServerError, "Failed to get connection from pool")
    })?;

    // Get the existing todo item from the database
    let target = todos::table.find(id);

    let existing_todo: Option<TodoItem> = target.first(&mut connection).optional()
    .map_err(|err| {
        error!("Failed to fetch todo: {:?}", err);
        (Status::InternalServerError, "Failed to fetch todo")
    })?;

    if existing_todo.is_none() {
        return Err((Status::NotFound, "Todo item not found"));
    }

    // Create updated data based on existing and new values
    let updated_data = NewTodoItem {
        title: updated_todo.title,
        completed: updated_todo.completed,
        user_id: updated_todo.user_id,
        description: updated_todo.description,
        priority: updated_todo.priority,
        due_date: updated_todo.due_date,
    };

    // Update the todo in the database and log any potential errors
    diesel::update(target)
        .set((
            todos::dsl::title.eq(updated_data.title),
            todos::dsl::completed.eq(updated_data.completed),
        ))
        .execute(&mut connection)
        .map_err(|err| {
            error!("Failed to update todo in the database: {:?}", err);
            (Status::InternalServerError, "Failed to update todo")
        })?;

    Ok("Todo updated successfully!")
}

// Mark a to-do item as completed
#[put("/todos/<id>/complete")]
pub fn complete_todo(pool: &State<DbPool>, id: i32) -> Result<&'static str, (Status, &'static str)> {
    info!("Marking to-do item with id: {} as completed", id);
    let mut connection = pool.get().map_err(|_| (Status::InternalServerError, "Failed to get connection from pool"))?;

    // Update the completed status of the todo
    diesel::update(todos::table.find(id))
        .set(todos::dsl::completed.eq(true))
        .execute(&mut connection)
        .map_err(|_| (Status::InternalServerError, "Failed to complete todo"))?;

    Ok("Todo marked as completed!")
}

#[get("/todos/search?<query>&<user_id>")]
pub fn search_todos(pool: &State<DbPool>, query: Option<String>, user_id: i32) -> Result<Json<Vec<TodoItem>>, (Status, &'static str)> {
    let mut connection = pool.get().map_err(|_| (Status::InternalServerError, "Failed to get connection from pool"))?;

    let results: Vec<TodoItem> = if let Some(query) = query {
        todos::table
            .filter(todos::dsl::title.like(format!("%{}%", query)))  // Search by title
            .filter(todos::dsl::user_id.eq(user_id))  // Ensure user_id matches
            .load(&mut connection)
            .map_err(|_| (Status::InternalServerError, "Failed to search todos"))?
    } else {
        todos::table
            .filter(todos::dsl::user_id.eq(user_id))  // Fetch todos only for the user
            .load(&mut connection)
            .map_err(|_| (Status::InternalServerError, "Failed to fetch todos"))?
    };

    Ok(Json(results))
}