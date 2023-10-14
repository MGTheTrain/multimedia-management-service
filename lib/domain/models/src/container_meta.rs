use crate::track::{VideoTrack, AudioTrack, SubtitleTrack};
use crate::model::Model;
use crate::schema::container_meta;
use crate::schema::video_track;
use crate::schema::audio_track;
use crate::schema::subtitle_track;
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
    pub video_track_id: Uuid,
    pub audio_track_id: Uuid,
    pub subtitle_track_id: Uuid,
    pub file_size_in_kb: i64,
    pub duration: f64,
}

impl Model for ContainerMeta {
    /// Method constructing a ContainerMeta object
    /// 
    /// Requires no paramters and returns a ContainerMeta object
    fn new() -> Self {
        ContainerMeta {
            id: Uuid::nil(),
            date_time_created: Utc::now(),
            date_time_updated: Utc::now(),
            title: String::from(""),
            description: String::from(""),
            tags: Vec::new(),
            video_track_id: Uuid::nil(),
            audio_track_id: Uuid::nil(),
            subtitle_track_id: Uuid::nil(),
            file_size_in_kb: 0,
            duration: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[test]
    fn test_container_meta() {
        let container_meta_id = Uuid::new_v4();

        let mut video_track = VideoTrack::new();
        video_track.id = Uuid::new_v4();
        video_track.name = String::from("simple_video.h264");
        video_track.container_meta_id = container_meta_id;
        video_track.media_type = String::from("h264");
        video_track.width = 1280;
        video_track.height = 720;
        video_track.bit_rate =  1835;
        video_track.frame_rate = 30;


        let mut audio_track = AudioTrack::new();
        audio_track.id = Uuid::new_v4();
        audio_track.name = String::from("simple_audio.aac");
        audio_track.container_meta_id = container_meta_id;
        audio_track.media_type = String::from("aac");
        audio_track.bit_rate = 157;
        audio_track.channel_config = String::from("stereo");
        audio_track.sample_frequenz =  48000;

        let mut subtitle_track = SubtitleTrack::new();
        subtitle_track.id = Uuid::new_v4();
        subtitle_track.name = String::from("simple_subtitle.unkown");
        subtitle_track.container_meta_id = container_meta_id;
        subtitle_track.media_type = String::from("unkown");

        // --
        let current_date_time = Utc::now();
        let tags: Vec<Option<String>> = vec![
            Some(String::from("entertainment")),
            Some(String::from("music")),
        ];

        let mut container_meta = ContainerMeta::new();
        container_meta.id = Uuid::new_v4();
        container_meta.date_time_created = current_date_time;
        container_meta.date_time_updated = current_date_time;
        container_meta.title = String::from("simple_container.mov");
        container_meta.description = String::from("This is a sample container with video and audio to be stored in Youtube or Netflix shared container platform");
        container_meta.tags = tags;
        container_meta.video_track_id = video_track.id;
        container_meta.audio_track_id = audio_track.id;
        container_meta.subtitle_track_id = subtitle_track.id;
        container_meta.file_size_in_kb = 100000;
        container_meta.duration = 200.23;

        assert_eq!(container_meta.date_time_created, current_date_time);
        assert_eq!(container_meta.date_time_updated, current_date_time);
        assert_eq!(container_meta.title, String::from("simple_container.mov"));
        assert_eq!(container_meta.description, "This is a sample container with video and audio to be stored in Youtube or Netflix shared container platform");
        assert_eq!(container_meta.tags.len(), 2);
        assert_eq!(container_meta.video_track_id, video_track.id);
        assert_eq!(container_meta.audio_track_id, audio_track.id);
        assert_eq!(container_meta.subtitle_track_id, subtitle_track.id);
        assert_eq!(container_meta.file_size_in_kb, 100000);
        assert_eq!(container_meta.duration, 200.23);
    }
}
