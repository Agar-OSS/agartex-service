CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    email VARCHAR(128) NOT NULL UNIQUE,
    password_hash VARCHAR(128) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE sessions (
    session_id CHAR(256) PRIMARY KEY,
    user_id INTEGER REFERENCES users(user_id),
    expires BIGINT NOT NULL
);
