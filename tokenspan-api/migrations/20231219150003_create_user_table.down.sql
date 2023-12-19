-- Add down migration script here
DROP INDEX IF EXISTS idx_user_username;
DROP INDEX IF EXISTS idx_user_email;

DROP TABLE IF EXISTS users;