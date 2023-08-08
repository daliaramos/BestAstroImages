-- Add up migration script here
INSERT INTO questions(title, content, tags, user_id) VALUES ('Question Title', 'Question Content', ARRAY['tag1', 'tag2']);