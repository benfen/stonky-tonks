-- Your SQL goes here
CREATE TABLE stockbuyoffers (
    id VARCHAR(128) NOT NULL PRIMARY KEY,
    userid VARCHAR(128) NOT NULL,
    stockid VARCHAR(6) NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    price INTEGER NOT NULL DEFAULT 0,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(userid) REFERENCES user(uuid),
    FOREIGN KEY(stockid) REFERENCES stockprice(symbol)
);

CREATE TABLE stockselloffers (
    id VARCHAR(128) NOT NULL PRIMARY KEY,
    userid VARCHAR(128) NOT NULL,
    stockid VARCHAR(6) NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    price INTEGER NOT NULL DEFAULT 0,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(userid) REFERENCES user(uuid),
    FOREIGN KEY(stockid) REFERENCES stockprice(symbol)
);

