-- Add up migration script here
CREATE TABLE play_user(
    user_id UUID PRIMARY KEY UNIQUE DEFAULT uuid_generate_v4(),
    hash TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    username VARCHAR(32) UNIQUE,
    verified BOOLEAN DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)