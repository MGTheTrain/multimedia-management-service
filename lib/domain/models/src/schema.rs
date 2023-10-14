// @generated automatically by Diesel CLI.

diesel::table! {
    audio_track (id) {
        id -> Uuid,
        container_meta_id -> Uuid,
        name -> Varchar,
        media_type -> Varchar,
        bit_rate -> Int4,
        channel_config -> Varchar,
        sample_frequenz -> Int4,
    }
}

diesel::table! {
    container_meta (id) {
        id -> Uuid,
        date_time_created -> Timestamptz,
        date_time_updated -> Timestamptz,
        title -> Varchar,
        description -> Varchar,
        tags -> Array<Nullable<Text>>,
        video_track_id -> Uuid,
        audio_track_id -> Uuid,
        subtitle_track_id -> Uuid,
        file_size_in_kb -> Int8,
        duration -> Float8,
    }
}

diesel::table! {
    subtitle_track (id) {
        id -> Uuid,
        container_meta_id -> Uuid,
        name -> Varchar,
        media_type -> Varchar,
    }
}

diesel::table! {
    video_track (id) {
        id -> Uuid,
        container_meta_id -> Uuid,
        name -> Varchar,
        media_type -> Varchar,
        width -> Int4,
        height -> Int4,
        bit_rate -> Int4,
        frame_rate -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    audio_track,
    container_meta,
    subtitle_track,
    video_track,
);
