

// See web::data example: https://github.com/actix/examples/blob/master/databases/diesel/src/main.rs

use actix_multipart::{Multipart, form::{MultipartForm, tempfile::TempFile}};
use actix_web::{HttpServer, web::{self, Bytes}, App, middleware, HttpResponse, post, Error, HttpResponseBuilder};

use std::{fs::File, fmt};
use std::io::Read;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

/// Upload a blob to a blob storage and insert metadata in SQL database table rows  
///
/// Requires an instance of `services::mutimedia_management_service::MutimediaManagementService`
#[post("/api/v1/mms/upload")]
async fn upload_blob(
    form: MultipartForm<UploadForm>,
    multimedia_management_service: web::Data<services::mutimedia_management_service::MutimediaManagementService>,
) -> Result<HttpResponse, actix_web::Error> {
    for f in &form.files {
        let mut upload_file_parameters = services::upload_parameters::UploadFileParameters::new();
        let file_name = f.file_name.clone().unwrap();
        upload_file_parameters.file_name = file_name.clone();
        let substrings: Vec<&str> = file_name.split('/').collect();

        if let Some(last_element) = substrings.last() {
            upload_file_parameters.blob_name = String::from(*last_element);
        } else {
            println!("No elements found.");
        }

        // Some mock data
        let mut upload_meta_parameters = services::upload_parameters::UploadMetaParameters::new();
        upload_meta_parameters.title = String::from("Peace");
        upload_meta_parameters.description = String::from("Peace for the world");
        upload_meta_parameters.tags = vec![Some(String::from("Nature"))];

        let result = multimedia_management_service
            .upload_blob_from_file_and_create_metadata(&upload_file_parameters, &upload_meta_parameters)
            .await;

        if let Err(err) = result {
            return Ok(HttpResponse::BadRequest().finish());
        }
    }
    Ok(HttpResponse::Ok().finish())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");

    let env_file_path = "./assets/app-secrets.dev.cfg";
    dotenv::from_path(env_file_path).ok();

    let multi_media_management_service = services::mutimedia_management_service::MutimediaManagementService::new().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(multi_media_management_service.clone()))
            // enable logger
            .wrap(middleware::Logger::default())
            .service(upload_blob)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}