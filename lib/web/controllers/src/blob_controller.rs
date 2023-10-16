

use axum::{
    extract::Multipart,
    routing::post,
    Router, http::StatusCode, response::Response, body::boxed,
};

struct BlobController {
    multi_media_management_service: Option<services::mutimedia_management_service::MutimediaManagementService>,
}

impl BlobController {
    /// Method for creating the BlobController constructor
    ///
    /// Requires no parameters and returns and BlobController object
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(BlobController {
            multi_media_management_service: Some(services::mutimedia_management_service::MutimediaManagementService::new().await),
        })
    }

    /// Method handle for uploading multipart files to a blob storage
    /// 
    ///  Requires &self and multipart as parameters and returns Result<Response, StatusCode>
    async fn upload(&self, mut multipart: Multipart) -> Result<Response, StatusCode> {
        while let Some(mut field) = multipart.next_field().await.unwrap() {
            let name = field.name().unwrap().to_string();
            let data = field.bytes().await.unwrap();
    
            // println!("Length of `{}` is {} bytes", name, data.len());
            let multi_media_management_service_unwrapped = self.multi_media_management_service.as_ref().unwrap();

            // temp mock data 
            let mut upload_file_parameters = services::upload_parameters::UploadFileParameters::new();
            upload_file_parameters.blob_name = String::from("nature2.mp4");
            upload_file_parameters.file_name = String::from("assets/nature2.mp4");
    
            let mut upload_meta_parameters = services::upload_parameters::UploadMetaParameters::new();
            upload_meta_parameters.title = String::from("Sample MP4 container file #001");
            upload_meta_parameters.description = String::from("A Sample MP4 container file");
            upload_meta_parameters.tags = vec![Some(String::from("nature")), Some(String::from("adventure"))];
            multi_media_management_service_unwrapped.upload_blob_from_bytes_and_create_metadata(
                &(data.len() as u64),&data.as_ref(), &upload_file_parameters, &upload_meta_parameters).await?;

            return Ok(Response::builder()
            .status(StatusCode::CREATED)
            .body(boxed("OK".to_string()))
            .unwrap());
        }
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


// let app = Router::new().route("/upload", post(upload));