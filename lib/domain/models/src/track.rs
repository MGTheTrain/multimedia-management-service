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
    pub media_type: String,
}

impl Model for Track {
    fn new() -> Self {
        Track {
            id: Uuid::new_v4(),
            container_meta_id: Uuid::new_v4(),
            name: String::from(""),
            file_type: 0,
            media_type: String::from(""),
        }
    }
}

// future task: consider splitting into audio, video and subtitle tracks which inherit from `Track` struct to gather more information
// See: https://github.com/alfg/mp4-rust/blob/master/examples/mp4info.rs
// command: cargo run --example mp4info  D:\videos\earth.mp4  
// File:
//   file size:          24406814
//   major_brand:        M4V
//   compatible_brands:  M4V  mp42 isom
// Movie:
//   version:        0
//   creation time:  1439235748
//   duration:       97.834s
//   fragments:      false
//   timescale:      90000
// Found 2 Tracks
//   Track: #2(eng) Audio: aac (LC) (mp4a / 0x6D703461), 48000 Hz, stereo, 157 kb/s
//   Track: #1(eng) Video: h264 (Baseline) (avc1 / 0x61766331), 1280x720, 1835 kb/s, 30.00 fps

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
            media_type: String::from("h264"),
        });
        assert_eq!(video_track.name, String::from("simple_video.h264"));
        assert_eq!(video_track.file_type, TrackType::Video.to_i32());
        assert_eq!(video_track.media_type, String::from("h264"));

        track_type = TrackType::Audio;
        let audio_track = Box::new(Track {
            id: Uuid::new_v4(),
            container_meta_id: Uuid::new_v4(),
            name: String::from("simple_audio.aac"),
            file_type: track_type.to_i32(),
            media_type: String::from("aac"),
        });
        assert_eq!(audio_track.name, String::from("simple_audio.aac"));
        assert_eq!(audio_track.file_type, TrackType::Audio.to_i32());
        assert_eq!(audio_track.media_type, String::from("aac"));

    }
}
