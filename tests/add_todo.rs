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

    // Assume we are assigning this todo to user_id 1 (since it’s the first test user in the seed data)
    let new_todo = json!({
        "title": "Test Todo",
        "completed": false,
        "user_id": 1  // Assigning to user with id 1
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
        "completed": false,
        "user_id": 1  // Assigning to user with id 1
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
        "completed": true,
        "user_id": 1  // Assigning to user with id 1
    });

    let response = client.post("/todos")
        .header(ContentType::JSON)
        .body(new_todo_completed.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.into_string().unwrap(), "New todo item cannot be marked as completed");
}
