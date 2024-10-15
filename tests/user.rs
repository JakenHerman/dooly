use rocket::http::Status;
use dooly::helpers::{cleanup_database, establish_test_connection, run_seed_script, setup_rocket};
use serde_json::json;
use rocket::http::ContentType;

#[test]
fn test_create_user_valid() {
    cleanup_database(); // Clean up the database before starting the test
    let pool = establish_test_connection();
    run_seed_script(&pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Create a valid new user
    let new_user = json!({
        "username": "testuser",
        "password_hash": "hashed_password"
    });

    let response = client.post("/users")
        .header(ContentType::JSON)
        .body(new_user.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "User created successfully!");
}

#[test]
fn test_create_user_empty_username() {
    cleanup_database(); // Clean up the database before starting the test
    let pool = establish_test_connection();
    run_seed_script(&pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Create a new user with an empty username
    let new_user = json!({
        "username": "",
        "password_hash": "hashed_password"
    });

    let response = client.post("/users")
        .header(ContentType::JSON)
        .body(new_user.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.into_string().unwrap(), "Username cannot be empty");
}

#[test]
fn test_create_user_empty_password() {
    cleanup_database(); // Clean up the database before starting the test
    let pool = establish_test_connection();
    run_seed_script(&pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Create a new user with an empty password
    let new_user = json!({
        "username": "testuser",
        "password_hash": ""
    });

    let response = client.post("/users")
        .header(ContentType::JSON)
        .body(new_user.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.into_string().unwrap(), "Password cannot be empty");
}

#[test]
fn test_get_user_by_id() {
    cleanup_database(); // Clean up the database before starting the test
    let pool = establish_test_connection();
    run_seed_script(&pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // First, create a new user
    let new_user = json!({
        "username": "testuser",
        "password_hash": "hashed_password"
    });

    let response = client.post("/users")
        .header(ContentType::JSON)
        .body(new_user.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "User created successfully!");

    // Now fetch the user by id
    let response = client.get("/users/2") // 2, because 1 is what's seeded
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    let user: serde_json::Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();

    // Ensure that the user_id and username are correct, and password_hash is not included
    assert_eq!(user["id"], 2);
    assert_eq!(user["username"], "testuser");
    assert!(user.get("password_hash").is_none());  // Ensure password_hash is not included
}

#[test]
fn test_get_user_by_id_not_found() {
    cleanup_database(); // Clean up the database before starting the test
    let pool = establish_test_connection();
    run_seed_script(&pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Attempt to get a user with an id that doesn't exist
    let response = client.get("/users/999")
        .dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert!(response.into_string().unwrap().contains("User not found"));
}
