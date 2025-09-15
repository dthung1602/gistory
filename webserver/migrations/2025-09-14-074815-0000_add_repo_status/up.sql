-- Your SQL goes here
ALTER TABLE repo
    ADD COLUMN status VARCHAR NOT NULL default 'done';
