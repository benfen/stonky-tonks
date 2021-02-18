-- Your SQL goes here
CREATE TABLE IF NOT EXISTS stockprices (
    id CHARACTER(36) NOT NULL PRIMARY KEY,
    name VARCHAR(40),
    symbol VARCHAR(6),
    price VARCHAR(20)
);
