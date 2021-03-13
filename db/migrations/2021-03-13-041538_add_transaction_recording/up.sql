-- Your SQL goes here
CREATE TABLE IF NOT EXISTS transactions (
    id VARCHAR(128) NOT NULL PRIMARY KEY,
    userid VARCHAR(128) NOT NULL,
    stockid VARCHAR(6) NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    kind VARCHAR(20) NOT NULL,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(userid) REFERENCES user(uuid),
    FOREIGN KEY(stockid) REFERENCES stockprice(symbol)
);
