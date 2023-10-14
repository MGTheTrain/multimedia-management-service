use crate::schema::subtitle_track;
use crate::schema::video_track;
use crate::schema::audio_track;
use diesel::prelude::*;
use uuid::Uuid;

use crate::model::Model;

#[derive(Insertable, Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = video_track)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VideoTrack {
    pub id: Uuid,
    pub container_meta_id: Uuid,
    pub name: String,
    pub media_type: String,
    pub width: i32,
    pub height: i32,
    pub bit_rate: i32,
    pub frame_rate: i32,
}

impl Model for VideoTrack {
    fn new() -> Self {
        VideoTrack {
            id: Uuid::nil(),
            container_meta_id: Uuid::nil(),
            name: String::from(""),
            media_type: String::from(""),
            width: 0,
            height: 0,
            bit_rate: 0,
            frame_rate: 0,
        }
    }
}

#[derive(Insertable, Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = audio_track)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AudioTrack {
    pub id: Uuid,
    pub container_meta_id: Uuid,
    pub name: String,
    pub media_type: String,
    pub bit_rate: i32,
    pub channel_config: String,
    pub sample_frequenz: i32, // in hz
}

impl Model for AudioTrack {
    fn new() -> Self {
        AudioTrack {
            id: Uuid::nil(),
            container_meta_id: Uuid::nil(),
            name: String::from(""),
            media_type: String::from(""),
            bit_rate: 0,
            channel_config: String::from(""),
            sample_frequenz: 0, // in hz
        }
    }
}

#[derive(Insertable, Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = subtitle_track)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SubtitleTrack {
    pub id: Uuid,
    pub container_meta_id: Uuid,
    pub name: String,
    pub media_type: String,
}

impl Model for SubtitleTrack {
    fn new() -> Self {
        SubtitleTrack {
            id: Uuid::nil(),
            container_meta_id: Uuid::nil(),
            name: String::from(""),
            media_type: String::from(""),
        }
    }
}

// // future task: consider splitting into audio, video and subtitle tracks struct to gather more information
// // See: https://github.com/alfg/mp4-rust/blob/master/examples/mp4info.rs
// // command: cargo run --example mp4info  D:\videos\earth.mp4  
// // File:
// //   file size:          24406814
// //   major_brand:        M4V
// //   compatible_brands:  M4V  mp42 isom
// // Movie:
// //   version:        0
// //   creation time:  1439235748
// //   duration:       97.834s
// //   fragments:      false
// //   timescale:      90000
// // Found 2 Tracks
// //   Track: #2(eng) Audio: aac (LC) (mp4a / 0x6D703461), 48000 Hz, stereo, 157 kb/s
// //   Track: #1(eng) Video: h264 (Baseline) (avc1 / 0x61766331), 1280x720, 1835 kb/s, 30.00 fps