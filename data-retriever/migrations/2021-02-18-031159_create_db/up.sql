-- Your SQL goes here
CREATE TABLE IF NOT EXISTS stockprice (
    symbol VARCHAR(6) NOT NULL PRIMARY KEY,
    name VARCHAR(40) NOT NULL,
    price VARCHAR(20) NOT NULL
);
