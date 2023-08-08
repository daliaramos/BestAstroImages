CREATE TABLE IF NOT EXISTS users
(
    id          serial PRIMARY KEY,
    email       VARCHAR(255) UNIQUE NOT NULL,
    password    VARCHAR(255) NOT NULL,
    user_role   VARCHAR(50) NOT NULL DEFAULT 'user',
    status      VARCHAR(50) NOT NULL DEFAULT 'Good',
);