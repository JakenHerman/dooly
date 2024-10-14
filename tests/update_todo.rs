use rocket::http::Status;
use dooly::{db::TodoItem, helpers::{cleanup_database, establish_connection, run_seed_script, setup_rocket}};
use serde_json::json;
use rocket::http::ContentType;

#[test]
fn test_update_todo() {
    cleanup_database(); // Clean up the database before starting the test
    let mut connection = establish_connection();
    run_seed_script(&mut connection); // Seed the database with initial data

    let client = setup_rocket();

    // Create a new todo item to update
    let new_todo = json!({
        "title": "Initial Todo",
        "completed": false
    });

    let response = client.post("/todos")
        .header(ContentType::JSON)
        .body(new_todo.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Todo added successfully!");

    // Update the existing todo item
    let updated_todo = json!({
        "title": "Updated Todo Title",
        "completed": true
    });

    let response = client.put("/todos/1")
        .header(ContentType::JSON)
        .body(updated_todo.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Todo updated successfully!");

    // Fetch the updated todo to verify the changes
    let response = client.get("/todos")
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    let todos: Vec<TodoItem> = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert_eq!(todos[0].title, "Updated Todo Title");
    assert_eq!(todos[0].completed, true);
}