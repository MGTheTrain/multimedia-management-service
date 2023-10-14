use crate::enums::TrackType;
use crate::track::Track;
use crate::model::Model;
use crate::schema::container_meta;
use crate::schema::track;
use uuid::Uuid;

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
    pub track_ids: Vec<Option<Uuid>>,
    pub file_size_in_kb: i64,
    pub duration: i64,
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
            track_ids: Vec::new(),
            file_size_in_kb: 0,
            duration: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track() {
        let mut track_type = TrackType::Video;
        let mut video_track = Track::new();
        video_track.id = Uuid::new_v4();
        video_track.name = String::from("simple_video.h264");
        video_track.file_type = track_type.to_i32();

        track_type = TrackType::Audio;
        let mut audio_track = Track::new();
        video_track.id = Uuid::new_v4();
        audio_track.name = String::from("simple_audio.aac");
        audio_track.file_type = track_type.to_i32();

        // --
        let current_date_time = Utc::now();
        let tags: Vec<Option<String>> = vec![
            Some(String::from("entertainment")),
            Some(String::from("music")),
        ];
        let track_ids: Vec<Option<Uuid>> =
            vec![Some(video_track.id), Some(audio_track.id)];

        let mut container_meta = ContainerMeta::new();
        container_meta.id = Uuid::new_v4();
        container_meta.date_time_created = current_date_time;
        container_meta.date_time_updated = current_date_time;
        container_meta.title = String::from("simple_container.mov");
        container_meta.description = String::from("This is a sample container with video and audio to be stored in Youtube or Netflix shared container platform");
        container_meta.tags = tags;
        container_meta.track_ids = track_ids;
        container_meta.file_size_in_kb = 100000;
        container_meta.duration = 200.23;

        assert_eq!(container_meta.date_time_created, current_date_time);
        assert_eq!(container_meta.date_time_updated, current_date_time);
        assert_eq!(container_meta.title, String::from("simple_container.mov"));
        assert_eq!(container_meta.description, "This is a sample container with video and audio to be stored in Youtube or Netflix shared container platform");
        assert_eq!(container_meta.tags.len(), 2);
        assert_eq!(container_meta.track_ids.len(), 2);
        assert_eq!(container_meta.file_size_in_kb, 100000);
        assert_eq!(container_meta.duration, 200.23);
    }
}
