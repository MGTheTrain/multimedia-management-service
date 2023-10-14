extern crate connectors;
extern crate data_access;
extern crate models;
use models::schema::container_meta::date_time_created;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::upload_parameters;
struct MutimediaManagementService {
    pub connector: Option<connectors::aws_s3_bucket_connector::AwsS3BucketConnector>,
    pub data_access: Option<data_access::psql_data_access_async::PsqlDataAccess>,
}

impl MutimediaManagementService {
    /// Method for creating the MutimediaManagementService constructor
    ///
    /// Requires no parameters and returns and MutimediaManagementService object
    async fn new() -> Self {
        MutimediaManagementService {
            connector: Some(connectors::aws_s3_bucket_connector::AwsS3BucketConnector::new().await.unwrap()),
            data_access: Some(data_access::psql_data_access_async::PsqlDataAccess::new().await.unwrap()),
        }
    }

    /// Method for uploading blobs to a blob storage and inserting for the uploaded file file_meta rows into a Psql database table utilizing diesel ORM
    ///
    /// Requires a key, a file_name and returns a Result<models::file_meta::FileMeta, diesel::result::Error>
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
        // TBD
        // For now mock data

        // video data (h264)
        let mut file_meta_type = models::enums::FileMetaType::Video;
        let mut video_file_meta_id = Uuid::new_v4();

        let mut video_file_meta =  models::file_meta::FileMeta {
            id: video_file_meta_id,
            container_meta_id: container_meta_id,
            name: String::from("simple_video.h264"),
            file_type: file_meta_type.to_i32(),
            file_size_in_kb: 200000,
        };
        self.data_access
            .as_ref()
            .unwrap()
            .insert_file_meta(&video_file_meta).await?;

        // audio data (aac)
        let mut audio_file_meta_id = Uuid::new_v4();
        file_meta_type = models::enums::FileMetaType::Audio;
        let mut audio_file_meta = models::file_meta::FileMeta {
            id: audio_file_meta_id,
            container_meta_id: container_meta_id,
            name: String::from("simple_audio.aac"),
            file_type: file_meta_type.to_i32(),
            file_size_in_kb: 150000,
        };
        self.data_access
            .as_ref()
            .unwrap()
            .insert_file_meta(&audio_file_meta).await?;

        // container (mp4, mov)
        let current_date_time: DateTime<Utc> = Utc::now();
        let file_meta_ids = vec![Some(video_file_meta_id), Some(audio_file_meta_id)];
        let container_meta = models::container_meta::ContainerMeta{
            id: container_meta_id,
            date_time_created: current_date_time,
            date_time_updated: current_date_time,
            title: upload_meta_parameters.title.clone(), // e.g. String::from("simple.mp4")
            description: upload_meta_parameters.description.clone(),
            tags: upload_meta_parameters.tags.clone(),
            file_meta_ids: file_meta_ids,
        };
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