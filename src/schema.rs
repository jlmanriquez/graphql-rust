table! {
    links (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        userid -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(links -> users (userid));

allow_tables_to_appear_in_same_query!(
    links,
    users,
);
