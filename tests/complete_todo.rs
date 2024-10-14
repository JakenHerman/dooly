use rocket::http::Status;
use dooly::{db::TodoItem, helpers::{cleanup_database, establish_connection, run_seed_script, setup_rocket}};
use serde_json::json;
use rocket::http::ContentType;

#[test]
fn test_complete_todo() {
    cleanup_database(); // Clean up the database before starting the test
    let mut connection = establish_connection();
    run_seed_script(&mut connection); // Seed the database with initial data

    let client = setup_rocket();

    // Create a new todo item
    let new_todo = json!({
        "title": "Test Incomplete Todo",
        "completed": false
    });

    let response = client.post("/todos")
        .header(ContentType::JSON)
        .body(new_todo.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Todo added successfully!");

    // Fetch the todo items to get the ID of the newly added todo
    let response = client.get("/todos")
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    let todos: Vec<TodoItem> = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    let todo_id = todos[0].id;  // Assuming this is the only todo item

    // Mark the todo item as completed
    let response = client.put(format!("/todos/{}/complete", todo_id))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Todo marked as completed!");

    // Fetch the updated todo to verify the changes
    let response = client.get("/todos")
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    let todos: Vec<TodoItem> = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert_eq!(todos[0].completed, true);
}
