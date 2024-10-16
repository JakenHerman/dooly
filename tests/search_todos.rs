use rocket::http::Status;
use dooly::helpers::{cleanup_database, establish_test_connection, run_seed_script, setup_rocket};
use serde_json::json;
use rocket::http::ContentType;

#[test]
fn test_search_todos_with_results() {
    let mut pool = establish_test_connection();
    cleanup_database(&mut pool).unwrap(); // Clean up the database before starting the test
    run_seed_script(&mut pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Create a few todos with user_id 1
    let new_todo_1 = json!({
        "title": "Test Todo 1",
        "completed": false,
        "user_id": 1
    });

    let new_todo_2 = json!({
        "title": "Important Task",
        "completed": false,
        "user_id": 1
    });

    client.post("/todos")
        .header(ContentType::JSON)
        .body(new_todo_1.to_string())
        .dispatch();

    client.post("/todos")
        .header(ContentType::JSON)
        .body(new_todo_2.to_string())
        .dispatch();

    // Search for todos with user_id 1 containing "Task"
    let response = client.get("/todos/search?query=Task&user_id=1").dispatch();
    
    assert_eq!(response.status(), Status::Ok);
    let todos: Vec<serde_json::Value> = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0]["title"], "Important Task");
}


#[test]
fn test_search_todos_no_results() {
    let mut pool = establish_test_connection();
    cleanup_database(&mut pool).unwrap(); // Clean up the database before starting the test
    run_seed_script(&mut pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Create a todo with user_id 1
    let new_todo = json!({
        "title": "Unrelated Task",
        "completed": false,
        "user_id": 1
    });

    client.post("/todos")
        .header(ContentType::JSON)
        .body(new_todo.to_string())
        .dispatch();

    // Search for todos with user_id 1 and query that doesn't match
    let response = client.get("/todos/search?query=NotInDatabase&user_id=1").dispatch();
    
    assert_eq!(response.status(), Status::Ok);
    let todos: Vec<serde_json::Value> = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    
    assert_eq!(todos.len(), 0);
}

#[test]
fn test_search_todos_no_query() {
    let mut pool = establish_test_connection();
    cleanup_database(&mut pool).unwrap(); // Clean up the database before starting the test
    run_seed_script(&mut pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Create a todo with user_id 1
    let new_todo = json!({
        "title": "General Todo",
        "completed": false,
        "user_id": 1
    });

    client.post("/todos")
        .header(ContentType::JSON)
        .body(new_todo.to_string())
        .dispatch();

    // Search without a query, but for user_id 1 (should return all todos for user_id 1)
    let response = client.get("/todos/search?user_id=1").dispatch();
    
    assert_eq!(response.status(), Status::Ok);
    let todos: Vec<serde_json::Value> = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    
    assert_eq!(todos.len(), 3); // 2 seeded + 1 new added
    assert_eq!(todos[2]["title"], "General Todo");
}

#[test]
fn test_search_todos_wrong_user() {
    let mut pool = establish_test_connection();
    cleanup_database(&mut pool).unwrap(); // Clean up the database before starting the test
    run_seed_script(&mut pool).unwrap(); // Seed the database with initial data

    let client = setup_rocket();

    // Create a todo with user_id 1
    let new_todo = json!({
        "title": "User 1's Todo",
        "completed": false,
        "user_id": 1
    });

    client.post("/todos")
        .header(ContentType::JSON)
        .body(new_todo.to_string())
        .dispatch();

    // Attempt to search with a different user_id (e.g., user_id=2)
    let response = client.get("/todos/search?user_id=2").dispatch();
    
    assert_eq!(response.status(), Status::Ok);
    let todos: Vec<serde_json::Value> = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    
    assert_eq!(todos.len(), 0);  // Should return no results for user_id=2
}