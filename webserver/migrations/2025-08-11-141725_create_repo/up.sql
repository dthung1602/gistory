-- Your SQL goes here
CREATE TABLE repo
(
    uuid     VARCHAR NOT NULL PRIMARY KEY,
    name     VARCHAR NOT NULL,
    username VARCHAR NOT NULL,
    email    VARCHAR NOT NULL,
    branch   VARCHAR NOT NULL,
    method   INTEGER NOT NULL
);
