use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

use mp4::{Error, Mp4Track, Result, TrackType};
use models::{
    container_meta::ContainerMeta,
    track::{self, VideoTrack, AudioTrack, SubtitleTrack},
};

pub struct Mp4Parser {} 

impl Mp4Parser {
    /// Method for retrieving MP4 file info
    ///
    /// Requires the filename as a parameter and returns a 
    /// Result<(ContainerMeta, VideoTrack, AudioTrack, SubtitleTrack), Box<dyn std::error::Error>> object   
    fn info(filename: &str) -> Result<(ContainerMeta, VideoTrack, AudioTrack, SubtitleTrack), Box<dyn std::error::Error>> {
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

        //tracks
        for track in mp4.tracks().values() {
            let media_info = match track.track_type()? {
                TrackType::Video => video_info(track),
                TrackType::Audio => audio_info(track),
                TrackType::Subtitle => subtitle_info(track),
            };
        }


        // container_meta.video_track_id = Uuid::nil(),
        // container_meta.audio_track_id = Uuid::nil(),
        // container_meta.subtitle_track_id = Uuid::nil(),

        Ok(container_meta,)
    }

    /// Helper method for the video section of the MP4 file if available
    ///
    /// Requires the filename as a parameter and returns a 
    /// Result<Option<VideoTrack>, Box<dyn std::error::Error>> object   
    fn video_info(track: &Mp4Track, video_track: &VideoTrack) -> Result<Option<VideoTrack>, Box<dyn std::error::Error>> {
        let video_track = VideoTrack::new();
        video_track.media_type = track.media_type()?;
        video_track.width = track.width();
        video_track.height = track.height();
        video_track.bit_rate = track.bitrate() / 1000;
        video_track.frame_rate = track.frame_rate();
        
        Ok(None)
    }

    
    /// Helper method for the audio section of the MP4 file if available
    ///
    /// Requires the filename as a parameter and returns a 
    /// Result<Option<AudioTrack>, Box<dyn std::error::Error>> object   
    fn audio_info(track: &Mp4Track, audio_track: &AudioTrack) -> Result<Option<AudioTrack>, Box<dyn std::error::Error>> {
        let audio_track = AudioTrack::new();
        audio_track.media_type = track.media_type()?;        
        audio_track.bit_rate = track.bitrate() / 1000;   
        let channel_config = match track.channel_config() {
            Ok(val) => val.to_string(),
            _ => "-".to_string(),
        };     
        audio_track.channel_config = channel_config;
        audio_track.sample_frequenz = track.sample_freq_index()?.freq();
        
        Ok(None)
    }

    /// Helper method for the subtitle section of the MP4 file if available
    /// 
    /// Requires the filename as a parameter and returns a 
    /// Result<Option<SubtitleTrack>, Box<dyn std::error::Error>> object   
    fn audio_info(track: &Mp4Track, subtitle_track: &SubtitleTrack) -> Result<Option<SubtitleTrack>, Box<dyn std::error::Error>> {
        let subtitle_track = SubtitleTrack::new();
        subtitle_track.media_type = track.media_type()?;        

        Ok(None)
    }
}