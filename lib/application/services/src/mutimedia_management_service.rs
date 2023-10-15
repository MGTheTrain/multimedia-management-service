extern crate connectors;
extern crate parsers;
extern crate data_access;
extern crate models;
use models::schema::container_meta::date_time_created;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::upload_parameters;
struct MutimediaManagementService {
    pub connector: Option<connectors::aws_s3_bucket_connector::AwsS3BucketConnector>,
    pub mp4_parser: Option<parsers::mp4_parser::Mp4Parser>,
    pub data_access: Option<data_access::psql_data_access_async::PsqlDataAccess>,
}

impl MutimediaManagementService {
    /// Method for creating the MutimediaManagementService constructor
    ///
    /// Requires no parameters and returns and MutimediaManagementService object
    async fn new() -> Self {
        MutimediaManagementService {
            connector: Some(connectors::aws_s3_bucket_connector::AwsS3BucketConnector::new().await.unwrap()),
            mp4_parser: Some(parsers::mp4_parser::Mp4Parser::new()),
            data_access: Some(data_access::psql_data_access_async::PsqlDataAccess::new().await.unwrap()),
        }
    }

    /// Method for uploading blobs to a blob storage and 
    /// inserting metadata in Psql database tables
    ///
    /// Requires upload_blob_parameters, upload_meta_parameters and returns a Result<(), Box<dyn std::error::Error>>
    async fn upload(        
        &self,
        upload_blob_parameters: &upload_parameters::UploadBlobParameters,
        upload_meta_parameters: &upload_parameters::UploadMetaParameters) -> Result<(), Box<dyn std::error::Error>> {
        
        let mut container_meta_id = Uuid::new_v4(); // leading element

        self.connector
            .as_ref()
            .unwrap()
            .upload_blob(&upload_blob_parameters.key, &upload_blob_parameters.file_name)
            .await?;
        
        // Extract information from the MP4, MOV container
        let (mut container_meta, video_track, audio_track, subtitle_track) = 
            self.mp4_parser.as_ref().unwrap().parse(&upload_blob_parameters.file_name).unwrap();

        // video data (h264)
        if(video_track != None){
            let mut video_track_unwrapped = video_track.unwrap(); 
            video_track_unwrapped.id = Uuid::new_v4();
            self.data_access
            .as_ref()
            .unwrap()
            .insert_video_track(&video_track_unwrapped).await?;
            container_meta.video_track_id = video_track_unwrapped.id;
        }

        // audio data (aac)
        if(audio_track != None){
            let mut audio_track_unwrapped = audio_track.unwrap(); 
            audio_track_unwrapped.id = Uuid::new_v4();
            self.data_access
            .as_ref()
            .unwrap()
            .insert_audio_track(&audio_track_unwrapped).await?;
            container_meta.audio_track_id = audio_track_unwrapped.id;
        }

        // subtitle
        if(subtitle_track != None){
            let mut subtitle_track_unwrapped = subtitle_track.unwrap(); 
            subtitle_track_unwrapped.id = Uuid::new_v4();
            self.data_access
            .as_ref()
            .unwrap()
            .insert_subtitlte_track(&subtitle_track_unwrapped).await?;
            container_meta.subtitle_track_id = subtitle_track_unwrapped.id;
        }

        // container (mp4, mov)
        container_meta.title = container_meta.title;
        container_meta.description = container_meta.description;
        container_meta.date_time_created = Utc::now();
        container_meta.date_time_updated = container_meta.date_time_created;
        container_meta.tags = upload_meta_parameters.tags.clone();

        self.data_access
        .as_ref()
        .unwrap()
        .insert_container_meta(&container_meta).await?;

        Ok(())
    }    

    async fn download(key: &str) -> Result<(), Box<dyn std::error::Error>> {

        Ok(())
    }    

    async fn delete(key: &str) -> Result<(), Box<dyn std::error::Error>> {

        Ok(())
    }    
}