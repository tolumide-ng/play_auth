-- Add up migration script here
CREATE TABLE play_user(
    user_id UUID PRIMARY KEY UNIQUE DEFAULT uuid_generate_v4(),
    hash VARCHAR(32),
    email VARCHAR(32) UNIQUE NOT NULL,
    verified BOOLEAN,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)