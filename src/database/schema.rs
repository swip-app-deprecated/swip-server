table! {
    parties (id) {
        id -> Integer,
        uuid -> Varchar,
        title -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        data -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        uuid -> Varchar,
        name -> Varchar,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users_parties (id) {
        id -> Integer,
        user_id -> Integer,
        party_id -> Integer,
    }
}

joinable!(users_parties -> parties (party_id));
joinable!(users_parties -> users (user_id));

allow_tables_to_appear_in_same_query!(
    parties,
    users,
    users_parties,
);
