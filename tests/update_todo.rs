use rocket::http::Status;
use dooly::{todos::TodoItem, helpers::{cleanup_database, establish_test_connection, run_seed_script, setup_rocket}};
use serde_json::json;
use rocket::http::ContentType;

#[test]
fn test_valid_update_todo() {
    let mut pool = establish_test_connection();
    cleanup_database(&mut pool).unwrap(); // Clean up the database before starting the test
    run_seed_script(&pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Create a new todo item to update
    let new_todo = json!({
        "title": "Initial Todo",
        "completed": false,
        "user_id": 1  // Assign to user with id 1
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
        "completed": true,
        "user_id": 1  // Ensure the user_id stays the same
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

#[test]
fn test_invalid_update_todo_empty_title() {
    let mut pool = establish_test_connection();  // Use pool now
    cleanup_database(&mut pool).unwrap(); // Clean up the database before starting the test
    run_seed_script(&pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Create a new todo item to update
    let new_todo = json!({
        "title": "Initial Todo",
        "completed": false,
        "user_id": 1  // Assign to user with id 1
    });

    let response = client.post("/todos")
        .header(ContentType::JSON)
        .body(new_todo.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Todo added successfully!");

    // Attempt to update the existing todo item with an empty title
    let updated_todo = json!({
        "title": "",
        "completed": true,
        "user_id": 1  // Ensure the user_id stays the same
    });

    let response = client.put("/todos/1")
        .header(ContentType::JSON)
        .body(updated_todo.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.into_string().unwrap(), "Title cannot be empty");
}
