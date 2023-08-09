CREATE TABLE IF NOT EXISTS questions
(
    id         serial PRIMARY KEY,
    title      VARCHAR(255) NOT NULL,
    content    TEXT         NOT NULL,
    created_on TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);