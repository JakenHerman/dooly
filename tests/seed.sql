CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL
);

INSERT INTO todos (title, completed) VALUES ('Test Todo 1', 0);
INSERT INTO todos (title, completed) VALUES ('Test Todo 2', 1);