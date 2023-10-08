use crate::schema::container_meta;
use crate::schema::file_meta;
use crate::enums::FileMetaType;
use crate::file_meta::FileMeta;
use crate::model::Model;
use uuid::Uuid;

extern crate chrono;

use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Insertable, Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = container_meta)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ContainerMeta {
    pub id: Uuid,
    pub date_time_created: DateTime<Utc>,
    pub date_time_updated: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub tags: Vec<Option<String>>,
    pub file_meta_ids: Vec<Option<Uuid>>,
}

impl Model for ContainerMeta {
    fn new() -> Self {
        ContainerMeta {
            id: Uuid::new_v4(),
            date_time_created: Utc::now(),
            date_time_updated: Utc::now(),
            title: String::from(""),
            description: String::from(""),
            tags: Vec::new(),
            file_meta_ids: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_meta() {
        let mut file_meta_type = FileMetaType::Video;
        let mut video_file_meta = FileMeta::new();
        video_file_meta.id = Uuid::new_v4();
        video_file_meta.name = String::from("simple_video.h264");
        video_file_meta.file_type = file_meta_type.to_i32();
        video_file_meta.file_size_in_kb = 200000;

        file_meta_type = FileMetaType::Audio;
        let mut audio_file_meta = FileMeta::new();
        video_file_meta.id = Uuid::new_v4();
        audio_file_meta.name = String::from("simple_audio.aac");
        audio_file_meta.file_type = file_meta_type.to_i32();
        audio_file_meta.file_size_in_kb = 150000;

        // --
        let current_date_time = Utc::now();
        let tags: Vec<Option<String>> = vec![Some(String::from("entertainment")), Some(String::from("music"))];
        let file_meta_ids: Vec<Option<i32>> = vec![Some(video_file_meta.id), Some(audio_file_meta.id)];

        let mut container_meta = ContainerMeta::new();
        container_meta.id = Uuid::new_v4();
        container_meta.date_time_created = current_date_time;
        container_meta.date_time_updated = current_date_time;
        container_meta.title = String::from("simple_container.mov");
        container_meta.description = String::from("This is a sample container with video and audio to be stored in Youtube or Netflix shared container platform");
        container_meta.tags = tags;
        container_meta.file_meta_ids = file_meta_ids;

        assert_eq!(container_meta.date_time_created, current_date_time);
        assert_eq!(container_meta.date_time_updated, current_date_time);
        assert_eq!(container_meta.title, String::from("simple_container.mov"));
        assert_eq!(container_meta.description, "This is a sample container with video and audio to be stored in Youtube or Netflix shared container platform");
        assert_eq!(container_meta.tags.len(), 2);
        assert_eq!(container_meta.file_meta_ids.len(), 2);
    }
}
