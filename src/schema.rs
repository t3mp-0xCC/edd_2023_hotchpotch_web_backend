// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Uuid,
        name -> Varchar,
        desc -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    joins (team_id, user_id) {
        team_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    requests (team_id, user_id) {
        team_id -> Uuid,
        user_id -> Uuid,
        #[max_length = 400]
        message -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    solos (event_id, user_id) {
        event_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    teams (id) {
        id -> Uuid,
        event_id -> Uuid,
        reader_id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 400]
        desc -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 400]
        icon_url -> Nullable<Varchar>,
        #[max_length = 400]
        profile -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(joins -> teams (team_id));
diesel::joinable!(joins -> users (user_id));
diesel::joinable!(requests -> teams (team_id));
diesel::joinable!(requests -> users (user_id));
diesel::joinable!(solos -> events (event_id));
diesel::joinable!(solos -> users (user_id));
diesel::joinable!(teams -> events (event_id));
diesel::joinable!(teams -> users (reader_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    joins,
    requests,
    solos,
    teams,
    users,
);
