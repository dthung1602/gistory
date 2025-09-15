-- This file should undo anything in `up.sql`
ALTER TABLE repo
    DROP COLUMN status;
