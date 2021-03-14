table! {
    stockbuyoffers (id) {
        id -> Text,
        userid -> Text,
        stockid -> Text,
        quantity -> Integer,
        price -> Integer,
        timestamp -> Timestamp,
    }
}

table! {
    stockholdings (userid, stockid) {
        userid -> Text,
        stockid -> Text,
        quantity -> Integer,
    }
}

table! {
    stockprice (symbol) {
        symbol -> Text,
        name -> Text,
        price -> Integer,
    }
}

table! {
    stockselloffers (id) {
        id -> Text,
        userid -> Text,
        stockid -> Text,
        quantity -> Integer,
        price -> Integer,
        timestamp -> Timestamp,
    }
}

table! {
    transactions (id) {
        id -> Text,
        userid -> Text,
        stockid -> Text,
        quantity -> Integer,
        kind -> Text,
        timestamp -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Text,
        name -> Text,
        capital -> BigInt,
    }
}

joinable!(stockbuyoffers -> stockprice (stockid));
joinable!(stockholdings -> stockprice (stockid));
joinable!(stockselloffers -> stockprice (stockid));
joinable!(transactions -> stockprice (stockid));

allow_tables_to_appear_in_same_query!(
    stockbuyoffers,
    stockholdings,
    stockprice,
    stockselloffers,
    transactions,
    user,
);
