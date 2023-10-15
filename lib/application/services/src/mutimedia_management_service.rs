extern crate connectors;
extern crate parsers;
extern crate data_access;
extern crate models;
use models::{schema::container_meta::date_time_created, model};
use uuid::Uuid;
use bytes::Bytes;
use chrono::{DateTime, Utc};

use crate::{upload_parameters, download_parameters};

struct MutimediaManagementService {
    pub blob_storage_connector: Option<connectors::aws_s3_bucket_connector::AwsS3BucketConnector>,
    pub mp4_parser: Option<parsers::mp4_parser::Mp4Parser>,
    pub sql_data_access: Option<data_access::psql_data_access_async::PsqlDataAccess>,
}

impl MutimediaManagementService {
    /// Method for creating the MutimediaManagementService constructor
    ///
    /// Requires no parameters and returns and MutimediaManagementService object
    pub async fn new() -> Self {
        MutimediaManagementService {
            blob_storage_connector: Some(connectors::aws_s3_bucket_connector::AwsS3BucketConnector::new().await.unwrap()),
            mp4_parser: Some(parsers::mp4_parser::Mp4Parser::new()),
            sql_data_access: Some(data_access::psql_data_access_async::PsqlDataAccess::new().await.unwrap()),
        }
    }

    /// Method for uploading blobs to a blob storage and 
    /// inserting metadata to relational database table rows
    ///
    /// Requires upload_blob_parameters, upload_meta_parameters and returns a Result<(), Box<dyn std::error::Error>>
    pub async fn upload_blob_and_create_metadata(        
        &self,
        upload_blob_parameters: &upload_parameters::UploadBlobParameters,
        upload_meta_parameters: &upload_parameters::UploadMetaParameters) -> Result<models::container_meta::ContainerMeta, Box<dyn std::error::Error>> {
        
        let mut container_meta_id = Uuid::new_v4(); // leading element

        self.blob_storage_connector
            .as_ref()
            .unwrap()
            .upload_blob(&upload_blob_parameters.blob_name, &upload_blob_parameters.file_name)
            .await?;
        
        // Parse information from the MP4, MOV container and assign attributes to the tuple members `let (mut container_meta, ...) = ...`
        let (mut container_meta, video_track, audio_track, subtitle_track) = 
            self.mp4_parser.as_ref().unwrap().parse(&upload_blob_parameters.file_name).unwrap();

        // video data (h264)
        if video_track != None {
            let mut video_track_unwrapped = video_track.unwrap(); 
            video_track_unwrapped.id = Uuid::new_v4();
            self.sql_data_access
            .as_ref()
            .unwrap()
            .insert_video_track(&video_track_unwrapped).await?;
            container_meta.video_track_id = video_track_unwrapped.id;
        }

        // audio data (aac)
        if audio_track != None {
            let mut audio_track_unwrapped = audio_track.unwrap(); 
            audio_track_unwrapped.id = Uuid::new_v4();
            self.sql_data_access
            .as_ref()
            .unwrap()
            .insert_audio_track(&audio_track_unwrapped).await?;
            container_meta.audio_track_id = audio_track_unwrapped.id;
        }

        // subtitle
        if subtitle_track != None {
            let mut subtitle_track_unwrapped = subtitle_track.unwrap(); 
            subtitle_track_unwrapped.id = Uuid::new_v4();
            self.sql_data_access
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

        self.sql_data_access
        .as_ref()
        .unwrap()
        .insert_container_meta(&container_meta).await?;

        Ok(container_meta)
    }    

    /// Method for downloading a blob from a blob storage
    ///
    /// Requires the &self and download_blob_parameters as parameters and 
    /// returns a Result<Bytes, Box<dyn std::error::Error>>
    pub async fn download_blob_by_name(&self, download_blob_parameters: &download_parameters::DownloadBlobParameters) 
        -> Result<Bytes, Box<dyn std::error::Error>> {
        let blob_name = String::from(&download_blob_parameters.container_meta_id) + 
            "/" + &download_blob_parameters.file_name; // <077cd041-45be-4699-8f54-c5c42c8298a3>/<sample.txt>
        let get_object_output = 
            self.blob_storage_connector.as_ref().unwrap().get_object(&blob_name).await?;
        let bytes = get_object_output
            .body
            .collect()
            .await
            .unwrap()
            .into_bytes(); 
        // let write_bytes_to_file_result = self.blob_storage_connector.as_ref().unwrap()
        //     .write_bytes_to_file(&bytes, download_file_path)
        //     .await;
        Ok(bytes)
    }    

    /// Method for deleting blobs from a blob storage and 
    /// deleting metadata inserted into relational database table rows by container_meta_id
    /// 
    /// Requires the &self, container_meta_id and file_name as parameters and 
    /// returns a Result<Bytes, Box<dyn std::error::Error>>
    pub async fn delete_blob_and_created_metadata_by_id(&self, container_meta_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.blob_storage_connector.as_ref().unwrap().delete_blob(&container_meta_id).await?;
        let uuid_from_str = Uuid::parse_str(container_meta_id).unwrap();

        // retrieve container_meta by container metaid
        let container_meta = 
            self.sql_data_access.as_ref().unwrap().get_container_meta_by_id(&uuid_from_str).await.unwrap();
        
        if container_meta.video_track_id != Uuid::nil() {
            self.sql_data_access.as_ref().unwrap().delete_video_track_by_id(&container_meta.video_track_id).await?;
        }
        if container_meta.audio_track_id != Uuid::nil() {
            self.sql_data_access.as_ref().unwrap().delete_audio_track_by_id(&container_meta.audio_track_id).await?;
        }
        if container_meta.subtitle_track_id != Uuid::nil() {
            self.sql_data_access.as_ref().unwrap().delete_subtitle_track_by_id(&container_meta.subtitle_track_id).await?;
        }

        self.sql_data_access.as_ref().unwrap().delete_container_meta_by_id(&uuid_from_str).await?;
        Ok(())
    }    

    /// Method for retrieving metadata inserted into relational database table rows by id
    /// 
    /// Requires the &self, id and file_name as parameters and 
    /// returns a Result<Option<models::ModelType>, Box<dyn std::error::Error>>
    pub async fn retrieve_metadata_by_id<T>(&self, id: &Uuid) -> Result<Option<models::ModelType>, Box<dyn std::error::Error>>
    where
        T: models::model::Model,
    {
        if std::any::type_name::<T>() == std::any::type_name::<models::container_meta::ContainerMeta>() {
            let model = self.sql_data_access
                .as_ref()
                .unwrap()
                .get_container_meta_by_id(&id)
                .await?;
            Ok(Some(models::ModelType::ContainerMeta(model)))
        } else if std::any::type_name::<T>() == std::any::type_name::<models::track::VideoTrack>() {
            let model = self.sql_data_access
                .as_ref()
                .unwrap()
                .get_video_track_by_id(&id)
                .await?;
            Ok(Some(models::ModelType::VideoTrack(model)))
        } else if std::any::type_name::<T>() == std::any::type_name::<models::track::AudioTrack>() {
            let model = self.sql_data_access
                .as_ref()
                .unwrap()
                .get_audio_track_by_id(&id)
                .await?;
            Ok(Some(models::ModelType::AudioTrack(model)))
        } else if std::any::type_name::<T>() == std::any::type_name::<models::track::SubtitleTrack>() {
            let model = self.sql_data_access
                .as_ref()
                .unwrap()
                .get_subtitle_track_by_id(&id)
                .await?;
            Ok(Some(models::ModelType::SubtitleTrack(model)))
        } 
        // Add similar branches for other model types
        else {
            // Err("Unknown model type".into())
            Ok(None)
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::upload_parameters::{UploadBlobParameters, UploadMetaParameters};

    use super::*;

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    pub async fn test_psql_data_access_methods_for_track() -> Result<(), Box<dyn std::error::Error>>{
        env_logger::init();
        
        let env_file_path = "./assets/app-secrets.dev.cfg";
        dotenv::from_path(env_file_path).ok();

        let multi_media_management_service = MutimediaManagementService::new().await;
        
        let mut upload_blob_parameters = UploadBlobParameters::new();
        upload_blob_parameters.blob_name = String::from("nature2.mp4");
        upload_blob_parameters.file_name = String::from("assets/nature2.mp4");

        let mut upload_meta_parameters = UploadMetaParameters::new();
        upload_meta_parameters.title = String::from("Sample MP4 container file #001");
        upload_meta_parameters.description = String::from("A Sample MP4 container file");
        upload_meta_parameters.tags = vec![Some(String::from("nature")), Some(String::from("adventure"))];
        
        let mut result = multi_media_management_service.upload_blob_and_create_metadata(&upload_blob_parameters, &upload_meta_parameters).await;
        assert!(result.is_ok());
        // // [C]reate
        // let mut result = psql_data_access.insert_video_track(&video_track).await;
        // assert!(result.is_ok());

        Ok(())
    }
}
