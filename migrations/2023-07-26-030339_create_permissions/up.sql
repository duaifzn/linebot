-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    group_id TEXT NOT NULL,
    in_the_group BOOLEAN NOT NULL DEFAULT FALSE,
    has_permission BOOLEAN NOT NULL DEFAULT FALSE,
    create_at TIMESTAMP DEFAULT current_timestamp,
    update_at TIMESTAMP DEFAULT current_timestamp
)