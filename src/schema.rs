table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

table! {
    vehicles (id) {
        id -> Int4,
        name -> Varchar,
        coolness -> Int4,
        wattage -> Int4,
        description -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    vehicles,
);
