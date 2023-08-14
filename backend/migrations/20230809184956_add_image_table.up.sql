-- Add up migration script here
CREATE TABLE IF NOT EXISTS images
(
    id                     serial PRIMARY KEY,
    copyright         TEXT      NOT NULL,
    explanation         TEXT      NOT NULL,
    hdurl                 TEXT      NOT NULL,
    media_type           TEXT      NOT NULL,
    service_version      TEXT      NOT NULL,
    title                 TEXT      NOT NULL,
    url                 TEXT      NOT NULL
);