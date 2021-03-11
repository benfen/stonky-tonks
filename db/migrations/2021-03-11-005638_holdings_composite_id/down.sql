-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS stockholdings;

CREATE TABLE IF NOT EXISTS stockholdings (
    uuid VARCHAR(128) NOT NULL PRIMARY KEY,
    userid VARCHAR(128) NOT NULL,
    stockid VARCHAR(6) NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(userid) REFERENCES user(uuid),
    FOREIGN KEY(stockid) REFERENCES stockprice(symbol)
);
