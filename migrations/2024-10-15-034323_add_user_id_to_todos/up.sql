-- Create a new table with the same structure as the original todos, plus the user_id column
CREATE TABLE todos_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    priority INTEGER,
    due_date DATE,
    completed BOOL NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)  -- Set up the foreign key relationship
);

-- Copy data from the old todos table into the new table
INSERT INTO todos_new (id, title, completed)
SELECT id, title, completed
FROM todos;

-- Drop the old todos table
DROP TABLE todos;

-- Rename the new table to todos
ALTER TABLE todos_new RENAME TO todos;
