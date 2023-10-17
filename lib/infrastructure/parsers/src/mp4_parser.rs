use std::env;
use std::fs::File;
use std::io::{prelude::*, Cursor};
use std::io::{self, BufReader};
use std::path::Path;
use chrono::{DateTime, Utc};
use models::container_meta::ContainerMeta;
use models::model::Model;
use models::track::{VideoTrack, AudioTrack, SubtitleTrack};
use uuid::Uuid;

use log::info;
use mp4::{Error, Mp4Track, TrackType};
use models;

#[derive(Clone)]
pub struct Mp4Parser {} 

impl Mp4Parser {
    /// Method constructing a Mp4Parser object
    /// 
    /// Requires no paramters and returns a Mp4Parser object
    pub fn new() -> Self {
        Mp4Parser {}
    }

    /// Method for retrieving MP4 file info and setting it in returned tuple
    ///
    /// Requires the &self and the filename as a parameter and returns a 
    /// Result<(ContainerMeta, Option<VideoTrack>, Option<AudioTrack>, Option<SubtitleTrack>), Box<dyn std::error::Error>>
    pub fn parse_from_file(&self, filename: &str) -> 
    Result<(ContainerMeta, Option<VideoTrack>, Option<AudioTrack>, Option<SubtitleTrack>), Box<dyn std::error::Error>> {
        let f = File::open(filename)?;
        let size = f.metadata()?.len();
        let reader = BufReader::new(f);

        // let bytes: &[u8] = b"Hello, world!"; 
        // let cursor = Cursor::new(bytes);
        // let mut reader = BufReader::new(cursor);

        let mp4:mp4::Mp4Reader<BufReader<File>>  = mp4::Mp4Reader::read_header(reader, size)?;

        let mut container_meta = ContainerMeta::new();
    
        container_meta.id = Uuid::new_v4();
        container_meta.date_time_created = Utc::now();
        container_meta.date_time_updated = container_meta.date_time_created;
        container_meta.file_size_in_kb = mp4.size() as i64;
        container_meta.duration = mp4.duration().as_secs() as f64;
        // container_meta title, description and tags need to be set

        // info!("File:");
        // info!("  file size:          {}", mp4.size());
        // info!("  major_brand:        {}", mp4.major_brand());
    
        let mut video_track: Option<VideoTrack> = None;
        let mut audio_track: Option<AudioTrack> = None;
        let mut subtitle_track: Option<SubtitleTrack> = None;
    
        // Tracks
        for track in mp4.tracks().values() {
            let media_info = match track.track_type()? {
                TrackType::Video => {
                    let track_id = Uuid::new_v4();
                    let mut video_info = self.get_video_info(track).unwrap();
                    video_info.id = track_id;
                    container_meta.video_track_id = track_id;
                    video_track = Some(video_info);
                },
                TrackType::Audio => {
                    let track_id = Uuid::new_v4();
                    let mut audio_info = self.get_audio_info(track).unwrap();
                    audio_info.id = track_id;
                    container_meta.audio_track_id = track_id;
                    audio_track = Some(audio_info);
                },
                TrackType::Subtitle => {
                    let track_id = Uuid::new_v4();
                    let mut subtitle_info = self.get_subtitle_info(track).unwrap();
                    subtitle_info.id = track_id;
                    container_meta.subtitle_track_id = track_id;
                    subtitle_track = Some(subtitle_info);
                }
            };
        }
        
        info!("Successfull parsed MP4 filename {}", filename ); 
        Ok((container_meta, video_track, audio_track, subtitle_track))
    }

    /// Helper method for the video section of the MP4 file if available
    ///
    /// Requires the &self and the track as a parameter and returns a 
    /// Result<VideoTrack, Box<dyn std::error::Error>> object   
    fn get_video_info(&self, track: &Mp4Track) -> Result<VideoTrack, Box<dyn std::error::Error>> {
        let mut video_track = models::track::VideoTrack::new();
        video_track.media_type = track.media_type()?.to_string() as String;
        video_track.width = track.width() as i32;
        video_track.height = track.height() as i32;
        video_track.bit_rate = (track.bitrate() / 1000) as i32;
        video_track.frame_rate = track.frame_rate() as i32;

        info!(
            "{} ({}) ({:?}), {}x{}, {} kb/s, {:.2} fps",
            track.media_type()?,
            track.video_profile()?,
            track.box_type()?,
            track.width(),
            track.height(),
            track.bitrate() / 1000,
            track.frame_rate()
        );
        
        Ok(video_track)
    }
    
    /// Helper method for the audio section of the MP4 file if available
    ///
    /// Requires the &self and the track as a parameter and returns a 
    /// Result<AudioTrack, Box<dyn std::error::Error>> object   
    fn get_audio_info(&self, track: &Mp4Track) -> Result<AudioTrack, Box<dyn std::error::Error>> {
        let mut audio_track = AudioTrack::new();
        audio_track.media_type = track.media_type()?.to_string() as String;        
        audio_track.bit_rate = (track.bitrate() / 1000) as i32;   

        let profile = match track.audio_profile() {
            Ok(val) => val.to_string(),
            _ => "-".to_string(),
        };

        let channel_config = match track.channel_config() {
            Ok(val) => val.to_string(),
            _ => "-".to_string(),
        };     
        audio_track.channel_config = channel_config;
        audio_track.sample_frequenz = track.sample_freq_index()?.freq() as i32;

        let channel_config = match track.channel_config() {
            Ok(val) => val.to_string(),
            _ => "-".to_string(),
        };

        info!(
            "{} ({}) ({:?}), {} Hz, {}, {} kb/s",
            track.media_type()?,
            profile,
            track.box_type()?,
            track.sample_freq_index()?.freq(),
            channel_config,
            track.bitrate() / 1000
        );
        
        Ok(audio_track)
    }

    /// Helper method for the subtitle section of the MP4 file if available
    /// 
    /// Requires the &self and the track as a parameter and returns a 
    /// Result<SubtitleTrack, Box<dyn std::error::Error>> object   
    fn get_subtitle_info(&self, track: &Mp4Track) -> Result<SubtitleTrack, Box<dyn std::error::Error>> {
        
        let mut subtitle_track = SubtitleTrack::new();
        subtitle_track.media_type = track.media_type()?.to_string() as String;        

        info!("{} ({:?})", track.media_type()?, track.box_type()?,);
        Ok(subtitle_track)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[test]
    fn test_mp4_parser() {
        env_logger::init();
        let mp4_parser = Mp4Parser::new();
        let (container_meta, video_track, audio_track, subtitle_track) = 
            mp4_parser.parse_from_file("assets/nature2.mp4").unwrap();
        assert_eq!(container_meta.file_size_in_kb, 5862561);
        // assert_eq!(container_meta.duration,);
        let video_track_unwrapped = video_track.unwrap(); 

        // video
        assert_eq!(video_track_unwrapped.media_type, "h264");
        assert_eq!(video_track_unwrapped.bit_rate, 1137);
        assert_eq!(video_track_unwrapped.frame_rate, 25);
        assert_eq!(video_track_unwrapped.width, 1280);
        assert_eq!(video_track_unwrapped.height, 720);

        // audio
        let audio_track_unwrapped = audio_track.unwrap(); 
        assert_eq!(audio_track_unwrapped.media_type, "aac");
        assert_eq!(audio_track_unwrapped.bit_rate, 0);
        assert_eq!(audio_track_unwrapped.sample_frequenz, 44100);

        // subtitle
        assert_eq!(subtitle_track, None);
    }
}
