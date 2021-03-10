-- Your SQL goes here
ALTER TABLE user
    RENAME COLUMN uuid TO id;

ALTER TABLE stockholdings
    RENAME COLUMN uuid TO id;
