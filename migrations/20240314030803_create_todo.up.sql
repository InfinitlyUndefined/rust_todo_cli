-- Add up migration script here
CREATE TABLE todo_items (
    id BIGSERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    complete BOOLEAN DEFAULT FALSE
)