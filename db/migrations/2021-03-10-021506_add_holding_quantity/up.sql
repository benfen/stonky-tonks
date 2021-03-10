-- Your SQL goes here
ALTER TABLE stockholdings
    ADD COLUMN quantity INTEGER NOT NULL DEFAULT 0;
