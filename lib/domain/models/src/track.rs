// The MIT License
// 
// Copyright (c) 2024 MGTheTrain
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


use crate::model::Model;
use crate::schema::subtitle_track;
use crate::schema::video_track;
use crate::schema::audio_track;
use diesel::prelude::*;
use uuid::Uuid;

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
    /// Method constructing a VideoTrack object
    /// 
    /// Requires no paramters and returns a VideoTrack object
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
    /// Method constructing a AudioTrack object
    /// 
    /// Requires no paramters and returns a AudioTrack object
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
    /// Method constructing a SubtitleTrack object
    /// 
    /// Requires no paramters and returns a SubtitleTrack object
    fn new() -> Self {
        SubtitleTrack {
            id: Uuid::nil(),
            container_meta_id: Uuid::nil(),
            name: String::from(""),
            media_type: String::from(""),
        }
    }
}

// See reference: https://github.com/alfg/mp4-rust/blob/master/examples/mp4info.rs
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