// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "file_meta_type"))]
    pub struct FileMetaType;
}

diesel::table! {
    container_meta (id) {
        id -> Int4,
        date_time_created -> Timestamptz,
        date_time_updated -> Timestamptz,
        title -> Varchar,
        description -> Varchar,
        tags -> Array<Nullable<Text>>,
        file_meta_ids -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FileMetaType;

    file_meta (id) {
        id -> Int4,
        name -> Varchar,
        file_type -> FileMetaType,
        file_size_in_kb -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    container_meta,
    file_meta,
);
