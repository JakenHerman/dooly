-- Recreate the original todos table without the user_id column
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    priority INTEGER,
    due_date DATE,
    completed BOOL NOT NULL
);

-- Copy the data back from the current todos table into the old structure
INSERT INTO todos (id, title, description, priority, due_date, completed)
SELECT id, title, description, priority, due_date, completed
FROM todos_new;

-- Drop the new table
DROP TABLE todos_new;
