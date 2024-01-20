-- Add up migration script here

-- create enum type for user role
CREATE TYPE user_role AS ENUM ('admin', 'user');

-- create user table
CREATE TABLE IF NOT EXISTS users
(
    id         uuid PRIMARY KEY,
    username   TEXT UNIQUE NOT NULL,
    password   TEXT        NOT NULL,
    email      TEXT UNIQUE NOT NULL,
    salt       TEXT        NOT NULL,
    role       user_role   NOT NULL,
    created_at TIMESTAMP   NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP   NOT NULL DEFAULT NOW()
);

ALTER TABLE users
    ADD search tsvector GENERATED ALWAYS AS
        (TO_TSVECTOR('simple', username) || ' ' || TO_TSVECTOR('simple', email) || ' ') STORED;

CREATE UNIQUE INDEX idx_user_username ON users (username);
CREATE UNIQUE INDEX idx_user_email ON users (email);
CREATE INDEX idx_user_created_at ON users (created_at, id);
CREATE INDEX idx_user_search ON users USING GIN(search);