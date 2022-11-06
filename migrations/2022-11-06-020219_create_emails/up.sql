-- Your SQL goes here
CREATE TABLE emails (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    create_at TIMESTAMP DEFAULT current_timestamp,
    update_at TIMESTAMP DEFAULT current_timestamp
)