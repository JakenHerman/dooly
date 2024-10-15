use rocket::http::Status;
use dooly::helpers::{establish_test_connection, setup_rocket, run_seed_script, cleanup_database};

#[test]
fn test_get_todos() {
    let mut pool = establish_test_connection();
    cleanup_database(&mut pool).unwrap(); // Clean up the database before starting the test
    run_seed_script(&mut pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Send a GET request to the /todos endpoint
    let response = client.get("/todos").dispatch();

    // Assert that the response status is OK (200)
    assert_eq!(response.status(), Status::Ok);

    // Parse the response body as JSON
    let todos: Vec<serde_json::Value> = serde_json::from_str(&response.into_string().unwrap()).expect("Failed to parse response as JSON");

    println!("Todos: {:?}", todos);
    // Assert that the correct number of todo items is returned
    assert_eq!(todos.len(), 2); // Adjust based on the number of items seeded

    // Assert that the first todo item has the expected properties, including user_id
    assert_eq!(todos[0]["title"], "Test Todo 1"); // Adjust based on seed data
    assert_eq!(todos[0]["completed"], false); // Adjust based on seed data
    assert_eq!(todos[0]["user_id"], 1); // Check that the user_id is correct

    // Assert that the second todo item has the expected properties, including user_id
    assert_eq!(todos[1]["title"], "Test Todo 2"); // Adjust based on seed data
    assert_eq!(todos[1]["completed"], true); // Adjust based on seed data
    assert_eq!(todos[1]["user_id"], 1); // Check that the user_id is correct
}
