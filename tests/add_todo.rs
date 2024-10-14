use rocket::http::Status;
use dooly::helpers::{establish_connection, setup_rocket, run_seed_script, cleanup_database};
use serde_json::json;
use rocket::http::ContentType;

#[test]
fn test_add_todo() {
    cleanup_database(); // Clean up the database before starting the test
    let mut connection = establish_connection();
    run_seed_script(&mut connection); // Seed the database with initial data

    let client = setup_rocket();

    // Create a new todo item
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