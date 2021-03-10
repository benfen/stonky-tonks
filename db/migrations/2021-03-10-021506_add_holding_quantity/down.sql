-- This file should undo anything in `up.sql`
ALTER TABLE stockholdings
    REMOVE COLUMN quantity;