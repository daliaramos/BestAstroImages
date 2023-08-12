-- Add up migration script here
CREATE TABLE IF NOT EXISTS comments
(
    id                     serial PRIMARY KEY,
    content                TEXT      NOT NULL,
    created_on             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    post_id            integer REFERENCES posts ON DELETE CASCADE
);