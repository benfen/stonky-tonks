-- Your SQL goes here
-- haha, oops; changing primary key kind of blows --
DROP TABLE IF EXISTS stockholdings;

CREATE TABLE stockholdings (
    userid VARCHAR(128) NOT NULL,
    stockid VARCHAR(6) NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY(userid, stockid),
    FOREIGN KEY(userid) REFERENCES user(uuid),
    FOREIGN KEY(stockid) REFERENCES stockprice(symbol)
);
