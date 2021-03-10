table! {
    stockholdings (id) {
        id -> Text,
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
    user (id) {
        id -> Text,
        name -> Text,
        capital -> BigInt,
    }
}

joinable!(stockholdings -> stockprice (stockid));
joinable!(stockholdings -> user (userid));

allow_tables_to_appear_in_same_query!(
    stockholdings,
    stockprice,
    user,
);
