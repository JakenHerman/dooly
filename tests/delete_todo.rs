use rocket::http::Status;
use dooly::helpers::{establish_connection, setup_rocket, run_seed_script, cleanup_database};

#[test]
fn test_delete_todo() {
    cleanup_database(); // Clean up the database before starting the test
    let mut connection = establish_connection();
    run_seed_script(&mut connection); // Seed the database with initial data

    let client = setup_rocket();

    // Assuming a todo item with ID 1 exists
    let response = client.delete("/todos/1").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Todo deleted successfully!");
}