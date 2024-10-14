#[macro_use] extern crate rocket;
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::http::Status;
use rocket::State;
use std::sync::{Mutex, PoisonError};

// Define the TodoItem struct with Serialize and Deserialize
#[derive(Serialize, Deserialize, Clone)]
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
}

// Global mutable list of to-do items
struct AppState {
    todos: Mutex<Vec<TodoItem>>,
}

// Initialize with some default to-dos
#[rocket::launch]
fn rocket() -> _ {
    let state = AppState {
        todos: Mutex::new(vec![
            TodoItem { id: 1, title: "Learn Rust".to_string(), completed: false },
            TodoItem { id: 2, title: "Build a REST API".to_string(), completed: false },
        ])
    };
    rocket::build()
        .manage(state)
        .mount("/", routes![get_todos, add_todo, update_todo, delete_todo])
}

// A helper function to convert Mutex lock errors into HTTP 500 responses.
fn handle_mutex_error<T>(_: PoisonError<T>) -> (Status, &'static str) {
    (Status::InternalServerError, "Failed to acquire lock on state")
}

// GET route for fetching to-do items
#[get("/todos")]
fn get_todos(state: &State<AppState>) -> Result<Json<Vec<TodoItem>>, (Status, &'static str)> {
    let todos = state.todos.lock().map_err(handle_mutex_error)?;
    Ok(Json(todos.clone()))
}

// POST route for adding a new to-do item
#[post("/todos", format = "json", data = "<new_todo>")]
fn add_todo(state: &State<AppState>, new_todo: Json<TodoItem>) -> Result<&'static str, (Status, &'static str)> {
    let mut todos = state.todos.lock().map_err(handle_mutex_error)?;
    todos.push(new_todo.into_inner());
    Ok("Todo added successfully!")
}

#[put("/todos/<id>", format = "json", data = "<updated_todo>")]
fn update_todo(
    state: &rocket::State<AppState>, 
    id: u32, 
    updated_todo: Json<TodoItem>
) -> Result<&'static str, (Status, &'static str)> {
    let mut todos = state.todos.lock().map_err(handle_mutex_error)?;

    // Find and update the matching to-do item
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
        *todo = updated_todo.into_inner();
        return Ok("Todo updated successfully!");
    }

    // Return a `NotFound` error if no matching todo was found
    Err((Status::NotFound, "Todo not found!"))
}

#[delete("/todos/<id>")]
fn delete_todo(
    state: &rocket::State<AppState>, 
    id: u32
) -> Result<&'static str, (Status, &'static str)> {
    let mut todos = state.todos.lock().map_err(handle_mutex_error)?;

    // Find the position of the to-do item with the given ID
    if let Some(pos) = todos.iter().position(|todo| todo.id == id) {
        todos.remove(pos);
        return Ok("Todo deleted successfully!");
    }

    // Return a `NotFound` error if no matching todo was found
    Err((Status::NotFound, "Todo not found!"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn test_get_todos() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/todos").dispatch();
        assert_eq!(response.status(), Status::Ok);
        
        let todos: Option<Vec<TodoItem>> = response.into_json().expect("valid json response");
        assert!(todos.is_some(), "Expected some todos");
        let todos = todos.unwrap(); // Unwrap here since we checked is_some
        assert_eq!(todos.len(), 2);
    }

    #[test]
    fn test_add_todo() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let new_todo = TodoItem { id: 3, title: "Write tests".to_string(), completed: false };
        
        let response = client.post("/todos")
            .json(&new_todo)
            .dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Todo added successfully!");

        // Verify the todo was added
        let response = client.get("/todos").dispatch();
        let todos: Option<Vec<TodoItem>> = response.into_json().expect("valid json response");
        assert!(todos.is_some(), "Expected some todos");
        let todos = todos.unwrap(); // Unwrap here since we checked is_some
        assert_eq!(todos.len(), 3);
    }

    #[test]
    fn test_update_todo() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let updated_todo = TodoItem { id: 1, title: "Learn Rust Programming".to_string(), completed: true };
        
        let response = client.put("/todos/1")
            .json(&updated_todo)
            .dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Todo updated successfully!");

        // Verify the todo was updated
        let response = client.get("/todos").dispatch();
        let todos: Option<Vec<TodoItem>> = response.into_json().expect("valid json response");
        assert!(todos.is_some(), "Expected some todos");
        let todos = todos.unwrap(); // Unwrap here since we checked is_some
        assert_eq!(todos[0].title, "Learn Rust Programming");
        assert!(todos[0].completed);
    }

    #[test]
    fn test_delete_todo() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        
        // Initially there should be 2 todos
        let initial_response = client.get("/todos").dispatch();
        let initial_todos: Option<Vec<TodoItem>> = initial_response.into_json().expect("valid json response");
        let initial_todos = initial_todos.unwrap();
        assert_eq!(initial_todos.len(), 2); // Confirm initial count
    
        // Now delete the second todo (id = 2)
        let response = client.delete("/todos/2").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Todo deleted successfully!");
    
        // Verify the todo was deleted
        let response = client.get("/todos").dispatch();
        let todos: Option<Vec<TodoItem>> = response.into_json().expect("valid json response");
        let todos = todos.unwrap(); // Unwrap here since we checked is_some
        assert_eq!(todos.len(), 1); // One todo should remain
        assert_eq!(todos[0].id, 1); // The remaining todo should be the one with id 1
    }
    

    #[test]
    fn test_update_nonexistent_todo() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let updated_todo = TodoItem { id: 999, title: "Nonexistent".to_string(), completed: false };

        let response = client.put("/todos/999")
            .json(&updated_todo)
            .dispatch();
        
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(response.into_string().unwrap(), "Todo not found!");
    }

    #[test]
    fn test_delete_nonexistent_todo() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client.delete("/todos/999").dispatch();
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(response.into_string().unwrap(), "Todo not found!");
    }
}
