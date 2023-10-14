use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use mp4::{Error, Mp4Track, TrackType};
use models::{
    container_meta::ContainerMeta,
    track::{self, VideoTrack, AudioTrack, SubtitleTrack},
};

pub struct Mp4Parser {} 

impl Mp4Parser {
    /// Method for retrieving MP4 file info
    ///
    /// Requires the &self and the filename as a parameter and returns a 
    /// Result<(ContainerMeta, Option<VideoTrack>, Option<AudioTrack>, Option<SubtitleTrack>), Box<dyn std::error::Error>>
    fn parse(&self, filename: &str) -> 
        Result<(ContainerMeta, Option<VideoTrack>, Option<AudioTrack>, Option<SubtitleTrack>), Box<dyn std::error::Error>> {
        let f = File::open(filename)?;
        let size = f.metadata()?.len();
        let reader = BufReader::new(f);

        let mp4 = mp4::Mp4Reader::read_header(reader, size)?;

        let container_meta = ContainerMeta::new();
        
        container_meta.id = Uuid::new_v4();
        container_meta.date_time_created = Utc::now();
        container_meta.date_time_updated = container_meta.date_time_created;
        container_meta.file_size_in_kb = mp4.size();
        container_meta.duration = mp4.duration();
        // container_meta.title = String::from(""); // not set here
        // container_meta.description = String::from(""); // not set here
        // container_meta.tags = Vec::new(); // not set here
        
        let video_track: Option<VideoTrack> = None;
        let audio_track: Option<AudioTrack> = None;
        let subtitle_track: Option<SubtitleTrack> = None;
        //tracks
        for track in mp4.tracks().values() {
            let media_info = match track.track_type()? {
                TrackType::Video => {
                    video_track = Some(self.get_video_info(track));
                    video_track.unwrap().id = Uuid::new_v4();
                },
                TrackType::Audio => {
                    audio_track = Some(self.get_audio_info(track));
                    audio_track.unwrap().id = Uuid::new_v4();
                },
                TrackType::Subtitle => {
                    subtitle_track = Some(self.get_subtitle_info(track));
                    subtitle_track.unwrap().id = Uuid::new_v4();
                }
            };
        }

        container_meta.video_track_id = video_track.unwrap().id;
        container_meta.audio_track_id = audio_track.unwrap().id;
        container_meta.subtitle_track_id = subtitle_track.unwrap().id;

        Ok((container_meta, video_track, audio_track, subtitle_track))
    }

    /// Helper method for the video section of the MP4 file if available
    ///
    /// Requires the track as a parameter and returns a 
    /// Result<VideoTrack, Box<dyn std::error::Error>> object   
    fn get_video_info(track: &Mp4Track) -> Result<VideoTrack, Box<dyn std::error::Error>> {
        let video_track = VideoTrack::new();
        video_track.media_type = track.media_type()?;
        video_track.width = track.width();
        video_track.height = track.height();
        video_track.bit_rate = track.bitrate() / 1000;
        video_track.frame_rate = track.frame_rate();
        
        Ok(video_track)
    }
    
    /// Helper method for the audio section of the MP4 file if available
    ///
    /// Requires the track as a parameter and returns a 
    /// Result<AudioTrack, Box<dyn std::error::Error>> object   
    fn get_audio_info(track: &Mp4Track) -> Result<AudioTrack, Box<dyn std::error::Error>> {
        let audio_track = AudioTrack::new();
        audio_track.media_type = track.media_type()?;        
        audio_track.bit_rate = track.bitrate() / 1000;   
        let channel_config = match track.channel_config() {
            Ok(val) => val.to_string(),
            _ => "-".to_string(),
        };     
        audio_track.channel_config = channel_config;
        audio_track.sample_frequenz = track.sample_freq_index()?.freq();
        
        Ok(audio_track)
    }

    /// Helper method for the subtitle section of the MP4 file if available
    /// 
    /// Requires the track as a parameter and returns a 
    /// Result<SubtitleTrack, Box<dyn std::error::Error>> object   
    fn get_subtitle_info(track: &Mp4Track) -> Result<SubtitleTrack, Box<dyn std::error::Error>> {
        let subtitle_track = SubtitleTrack::new();
        subtitle_track.media_type = track.media_type()?;        

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
        let (container_meta, video_track, audio_track, subtitle_track) = mp4_parser.parse("assets/nature2.mp4")?;
    }
}
