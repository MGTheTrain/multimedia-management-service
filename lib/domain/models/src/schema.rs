// @generated automatically by Diesel CLI.

diesel::table! {
    container_meta (id) {
        id -> Uuid,
        date_time_created -> Timestamptz,
        date_time_updated -> Timestamptz,
        title -> Varchar,
        description -> Varchar,
        tags -> Array<Nullable<Text>>,
        track_ids -> Array<Nullable<Uuid>>,
        file_size_in_kb -> Int8,
        duration -> Int8,
    }
}

diesel::table! {
    track (id) {
        id -> Uuid,
        container_meta_id -> Uuid,
        name -> Varchar,
        file_type -> Int4,
        media_type -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    container_meta,
    track,
);
