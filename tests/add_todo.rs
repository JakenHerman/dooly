use rocket::http::Status;
use dooly::helpers::{establish_test_connection, setup_rocket, run_seed_script, cleanup_database};
use serde_json::json;
use rocket::http::ContentType;

#[test]
fn test_add_valid_todo() {
    cleanup_database(); // Clean up the database before starting the test
    let mut pool = establish_test_connection();
    run_seed_script(&mut pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Create a valid new todo item
    let new_todo = json!({
        "title": "Test Todo",
        "completed": false
    });

    let response = client.post("/todos")
        .header(ContentType::JSON)
        .body(new_todo.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Todo added successfully!");
}

#[test]
fn test_add_todo_empty_title() {
    cleanup_database(); // Clean up the database before starting the test
    let mut pool = establish_test_connection();
    run_seed_script(&mut pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Create a new todo item with an empty title
    let new_todo_empty_title = json!({
        "title": "",
        "completed": false
    });

    let response = client.post("/todos")
        .header(ContentType::JSON)
        .body(new_todo_empty_title.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.into_string().unwrap(), "Title cannot be empty");
}

#[test]
fn test_add_todo_marked_completed() {
    cleanup_database(); // Clean up the database before starting the test
    let mut pool = establish_test_connection();
    run_seed_script(&mut pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Create a new todo item that is marked as completed
    let new_todo_completed = json!({
        "title": "Test Todo 2",
        "completed": true
    });

    let response = client.post("/todos")
        .header(ContentType::JSON)
        .body(new_todo_completed.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.into_string().unwrap(), "New todo item cannot be marked as completed");
}