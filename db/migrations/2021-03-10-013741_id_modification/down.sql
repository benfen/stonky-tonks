-- This file should undo anything in `up.sql`
ALTER TABLE user
    RENAME COLUMN id TO uuid;

ALTER TABLE stockholdings
    RENAME COLUMN id TO uuid;
