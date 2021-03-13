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

joinable!(stockholdings -> stockprice (stockid));
joinable!(transactions -> stockprice (stockid));

allow_tables_to_appear_in_same_query!(
    stockholdings,
    stockprice,
    transactions,
    user,
);
