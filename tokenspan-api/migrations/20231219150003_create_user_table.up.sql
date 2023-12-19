-- Add up migration script here
-- create enum type for user role
CREATE TYPE user_role AS ENUM ('ADMIN', 'USER');

-- create user table
CREATE TABLE users
(
    id         uuid PRIMARY KEY,
    username   text UNIQUE NOT NULL,
    password   text        NOT NULL,
    email      text UNIQUE NOT NULL,
    salt       text        NOT NULL,
    role       user_role   NOT NULL,
    created_at timestamp   NOT NULL DEFAULT NOW(),
    updated_at timestamp   NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_user_username ON users (username);
CREATE UNIQUE INDEX idx_user_email ON users (email);