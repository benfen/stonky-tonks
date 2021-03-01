-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user (
    uuid VARCHAR(128) NOT NULL PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE IF NOT EXISTS stockholdings (
    uuid VARCHAR(128) NOT NULL PRIMARY KEY,
    userid VARCHAR(128) NOT NULL,
    stockid VARCHAR(6) NOT NULL,
    FOREIGN KEY(userid) REFERENCES user(uuid),
    FOREIGN KEY(stockid) REFERENCES stockprice(symbol)
);
