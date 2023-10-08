// @generated automatically by Diesel CLI.

diesel::table! {
    container_meta (id) {
        id -> Uuid,
        date_time_created -> Timestamptz,
        date_time_updated -> Timestamptz,
        title -> Varchar,
        description -> Varchar,
        tags -> Array<Nullable<Text>>,
        file_meta_ids -> Array<Nullable<Uuid>>,
    }
}

diesel::table! {
    file_meta (id) {
        id -> Uuid,
        container_id -> Uuid,
        name -> Varchar,
        file_type -> Int4,
        file_size_in_kb -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    container_meta,
    file_meta,
);
