extern crate connectors;
extern crate parsers;
extern crate data_access;
extern crate models;

use log::info;
use models::{schema::container_meta::date_time_created, model};
use uuid::Uuid;
use bytes::Bytes;
use chrono::{DateTime, Utc};

use crate::{upload_parameters, download_parameters, delete_parameters::{self, DeleteBlobParameters}};

#[derive(Clone)]
pub struct MutimediaManagementService {
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

    /// Method for uploading blobs from a file (which then will be converted to a bytestream) to a blob storage and 
    /// inserting metadata to relational database table rows
    ///
    /// Requires upload_file_parameters, upload_meta_parameters and returns a Result<models::container_meta::ContainerMeta, Box<dyn std::error::Error>>
    pub async fn upload_blob_from_file_and_create_metadata(        
        &self,
        upload_file_parameters: &upload_parameters::UploadFileParameters,
        upload_meta_parameters: &upload_parameters::UploadMetaParameters) -> Result<models::container_meta::ContainerMeta, Box<dyn std::error::Error>> {
        
        let container_meta_id = Uuid::new_v4(); // leading element

        let updated_blob_name = container_meta_id.to_string() + "/" + &upload_file_parameters.blob_name;
        self.blob_storage_connector
            .as_ref()
            .unwrap()
            .upload_blob(&updated_blob_name, &upload_file_parameters.file_name)
            .await?;
        
        // Parse information from the MP4, MOV container and assign attributes to the tuple members `let (mut container_meta, ...) = ...`
        let (mut container_meta, video_track, audio_track, subtitle_track) = 
            self.mp4_parser.as_ref().unwrap().parse_from_file(&upload_file_parameters.file_name).unwrap();

        // video data (h264)
        if video_track != None {
            let mut video_track_unwrapped = video_track.unwrap(); 
            video_track_unwrapped.id = Uuid::new_v4();
            video_track_unwrapped.container_meta_id = container_meta_id;
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
            audio_track_unwrapped.container_meta_id = container_meta_id;
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
            subtitle_track_unwrapped.container_meta_id = container_meta_id;
            self.sql_data_access
            .as_ref()
            .unwrap()
            .insert_subtitlte_track(&subtitle_track_unwrapped).await?;
            container_meta.subtitle_track_id = subtitle_track_unwrapped.id;
        }

        // container (mp4, mov)
        container_meta.id = container_meta_id;
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

    /// Method for retrieving bytes from blobs in a blob storage required for downloading 
    ///
    /// Requires the &self and download_blob_parameters as parameters and 
    /// returns a Result<Bytes, Box<dyn std::error::Error>>
    pub async fn retrieve_bytes_from_blob_by_name(&self, download_blob_parameters: &download_parameters::DownloadBlobParameters) 
        -> Result<Bytes, Box<dyn std::error::Error>> {
        let get_object_output = 
            self.blob_storage_connector.as_ref().unwrap().get_object(
                &download_blob_parameters.get_blob_name()).await?; // blob name example: <077cd041-45be-4699-8f54-c5c42c8298a3>/<sample.txt>
        let bytes = get_object_output
            .body
            .collect()
            .await
            .unwrap()
            .into_bytes(); 
        Ok(bytes)
    }    

    /// Method for deleting blobs from a blob storage and 
    /// deleting metadata inserted into relational database table rows by id
    /// 
    /// Requires the &self, delete_blob_parameters and file_name as parameters and 
    /// returns a Result<(), Box<dyn std::error::Error>>
    pub async fn delete_blob_and_created_metadata_by_id(&self, delete_blob_parameters: &DeleteBlobParameters) -> Result<(), Box<dyn std::error::Error>> {
        let blob_name = delete_blob_parameters.get_blob_name();
        self.blob_storage_connector.as_ref().unwrap().delete_blob(&delete_blob_parameters.get_blob_name()).await?; // delete blob
        self.blob_storage_connector.as_ref().unwrap().delete_blob(&delete_blob_parameters.container_meta_id).await?; // delete folder
        let uuid_from_str = Uuid::parse_str(&delete_blob_parameters.container_meta_id).unwrap();

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

    // /// [TOO COMPLEX] Method for updating metadata inserted into relational database table rows by id
    // /// 
    // /// Requires the &self, id and model_type as parameters and 
    // /// returns a Result<Option<models::ModelType>, Box<dyn std::error::Error>>
    // pub async fn update_metadata_by_id(&self, id: &Uuid, model_type: &models::ModelType) -> Result<Option<models::ModelType>, Box<dyn std::error::Error>>
    // {
    //     match model_type {
    //         models::ModelType::ContainerMeta(container_meta) => {
    //             let model = self.sql_data_access
    //                 .as_ref()
    //                 .unwrap()
    //                 .update_container_meta_by_id(&id, container_meta)
    //                 .await?;
    //             Ok(Some(models::ModelType::ContainerMeta(model)))
    //         }
    //         models::ModelType::VideoTrack(video_track) => {
    //             let model = self.sql_data_access
    //                 .as_ref()
    //                 .unwrap()
    //                 .update_video_track_by_id(&id, video_track)
    //                 .await?;
    //             Ok(Some(models::ModelType::VideoTrack(model)))
    //         }
    //         models::ModelType::AudioTrack(audio_track) => {
    //             let model = self.sql_data_access
    //                 .as_ref()
    //                 .unwrap()
    //                 .update_audio_track_by_id(&id, audio_track)
    //                 .await?;
    //             Ok(Some(models::ModelType::AudioTrack(model)))
    //         }
    //         models::ModelType::SubtitleTrack(subtitle_track) => {
    //             let model = self.sql_data_access
    //                 .as_ref()
    //                 .unwrap()
    //                 .update_subtitle_track_by_id(&id, subtitle_track)
    //                 .await?;
    //             Ok(Some(models::ModelType::SubtitleTrack(model)))
    //         }
    //         _ => {
    //             // Handle any other unhandled variants (catch-all)
    //             println!("Received an unknown variant");
    //             Ok(None)
    //         }
    //     }
    // }

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
    use models::{schema::{audio_track::container_meta_id, subtitle_track}, container_meta::ContainerMeta, model::Model};

    use crate::{upload_parameters::{UploadFileParameters, UploadMetaParameters}, download_parameters::DownloadBlobParameters, delete_parameters::DeleteBlobParameters};

    use super::*;

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    pub async fn test_psql_data_access_methods_for_track() -> Result<(), Box<dyn std::error::Error>>{
        env_logger::init();
        
        let env_file_path = "./assets/app-secrets.dev.cfg";
        dotenv::from_path(env_file_path).ok();

        let mut multi_media_management_service = MutimediaManagementService::new().await;

        let mut upload_file_parameters = UploadFileParameters::new();
        upload_file_parameters.blob_name = String::from("nature2.mp4");
        upload_file_parameters.file_name = String::from("assets/nature2.mp4");

        let mut upload_meta_parameters = UploadMetaParameters::new();
        upload_meta_parameters.title = String::from("Sample MP4 container file #001");
        upload_meta_parameters.description = String::from("A Sample MP4 container file");
        upload_meta_parameters.tags = vec![Some(String::from("nature")), Some(String::from("adventure"))];
        
        // [C]reate
        // Retrieve metadata
        let create_result = multi_media_management_service.upload_blob_from_file_and_create_metadata(&upload_file_parameters, &upload_meta_parameters).await;
        assert!(create_result.is_ok());
        let create_result_unwrapped = create_result.unwrap();

        // [R]ead
        let mut get_container_meta_result = 
            multi_media_management_service.retrieve_metadata_by_id::<models::container_meta::ContainerMeta>(&create_result_unwrapped.id).await;
        assert!(get_container_meta_result.is_ok());
        // let conainer_meta = get_result.unwrap().unwrap() as models::container_meta::ContainerMeta;
        let mut get_video_track_result = 
            multi_media_management_service.retrieve_metadata_by_id::<models::track::VideoTrack>(&create_result_unwrapped.video_track_id).await;
        assert!(get_video_track_result.is_ok());
        let get_video_track_result_unwrapped = get_video_track_result.unwrap(); 

        let mut get_audio_track_result = 
            multi_media_management_service.retrieve_metadata_by_id::<models::track::AudioTrack>(&create_result_unwrapped.audio_track_id).await;
        assert!(get_audio_track_result.is_ok());

        let mut get_subtitle_track_result =  
            multi_media_management_service.retrieve_metadata_by_id::<models::track::SubtitleTrack>(&create_result_unwrapped.subtitle_track_id).await;
        assert!(get_subtitle_track_result.is_err());

        // Download
        let mut download_blob_parameters = DownloadBlobParameters::new();
        download_blob_parameters.container_meta_id = create_result_unwrapped.id.to_string();
        download_blob_parameters.file_name = upload_file_parameters.blob_name;
        let download_result = multi_media_management_service.retrieve_bytes_from_blob_by_name(&download_blob_parameters).await;
        assert!(download_result.is_ok());
        let bytes = download_result.unwrap();

        assert!(bytes.len() > 0);
        let download_file_path = "temp/nature2-copy.mp4";
        let write_bytes_to_file_result = 
            multi_media_management_service.blob_storage_connector.as_ref().unwrap()
                .write_bytes_to_file(&bytes, download_file_path)
                .await;
        
        // [U]pdate
        let get_container_meta_result_unwrapped = get_container_meta_result.unwrap().unwrap();
        match get_container_meta_result_unwrapped {
            models::ModelType::ContainerMeta(container_meta) => {
            }
            models::ModelType::VideoTrack(video_track) => {
                let mut updated_video_track = models::track::VideoTrack::new();

                updated_video_track = updated_video_track; 
                updated_video_track.width = 1290;
                
                let multi_media_management_service_sql_data_access_unwrapped = multi_media_management_service.sql_data_access.unwrap();
                let update_result = multi_media_management_service_sql_data_access_unwrapped
                .update_video_track_by_id(&create_result_unwrapped.video_track_id, &updated_video_track).await;

                assert!(update_result.is_ok());
            }
            models::ModelType::AudioTrack(audio_track) => {
                // Handle AudioTrack
            }
            models::ModelType::SubtitleTrack(subtitle_track) => {
                // Handle SubtitleTrack
            }
            _ => {
                println!("Received an unknown variant");
            }
        }
        // [D]elete
        multi_media_management_service = MutimediaManagementService::new().await;
        let mut delete_blob_parameters = DeleteBlobParameters::new();
        delete_blob_parameters.container_meta_id = create_result_unwrapped.id.to_string();
        delete_blob_parameters.file_name = download_blob_parameters.file_name;
        let delete_result = multi_media_management_service.delete_blob_and_created_metadata_by_id(&delete_blob_parameters).await;
        assert!(delete_result.is_ok());

        Ok(())
    }
}
