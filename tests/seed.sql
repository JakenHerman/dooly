-- Create users table if it doesn't exist
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

-- Insert a test user into the users table
INSERT INTO users (username, password_hash) VALUES ('test_user', 'hashed_password');

-- Create todos table if it doesn't exist
CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    priority INTEGER,
    due_date DATE,
    completed BOOLEAN NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Insert test todos and assign them to the test user
-- Assume the test user has id 1 (because it’s the first user inserted)
INSERT INTO todos (title, completed, user_id) VALUES ('Test Todo 1', 0, 1);
INSERT INTO todos (title, completed, user_id) VALUES ('Test Todo 2', 1, 1);
