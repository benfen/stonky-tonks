-- Your SQL goes here
CREATE TABLE IF NOT EXISTS stockprice (
    id VARCHAR(128) NOT NULL PRIMARY KEY,
    name VARCHAR(40) NOT NULL,
    symbol VARCHAR(6) NOT NULL,
    price VARCHAR(20) NOT NULL
);
