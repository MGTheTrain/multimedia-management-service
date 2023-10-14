use crate::schema::track;
use diesel::prelude::*;
use uuid::Uuid;

use crate::enums::TrackType;
use crate::model::Model;

#[derive(Insertable, Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = track)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Track {
    pub id: Uuid,
    pub container_meta_id: Uuid,
    pub name: String,
    pub file_type: i32, // utilize TrackType enum for file_type
}

impl Model for Track {
    fn new() -> Self {
        Track {
            id: Uuid::new_v4(),
            container_meta_id: Uuid::new_v4(),
            name: String::from(""),
            file_type: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track() {
        // Track::new() when wanting to create a file meta object trough its constructor on the stack memory
        let mut track_type = TrackType::Video;
        let video_track = Box::new(Track {
            id: Uuid::new_v4(),
            container_meta_id: Uuid::new_v4(),
            name: String::from("simple_video.h264"),
            file_type: track_type.to_i32(),
        });
        assert_eq!(video_track.name, String::from("simple_video.h264"));
        assert_eq!(video_track.file_type, TrackType::Video.to_i32());

        track_type = TrackType::Audio;
        let audio_track = Box::new(Track {
            id: Uuid::new_v4(),
            container_meta_id: Uuid::new_v4(),
            name: String::from("simple_audio.aac"),
            file_type: track_type.to_i32(),
        });
        assert_eq!(audio_track.name, String::from("simple_audio.aac"));
        assert_eq!(audio_track.file_type, TrackType::Audio.to_i32());
    }
}
