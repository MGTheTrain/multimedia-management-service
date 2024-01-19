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
//
// Maintainers:
// - MGTheTrain
//
// Contributors:
// - TBD

// See web::data example: https://github.com/actix/examples/blob/master/databases/diesel/src/main.rs

use actix_multipart::{
    form::{tempfile::TempFile, MultipartForm},
    Multipart,
};
use actix_web::{
    middleware, post,
    web::{self, Bytes},
    App, Error, HttpResponse, HttpResponseBuilder, HttpServer,
};

use std::io::Read;
use std::{
    fmt,
    fs::{self, File},
};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/api/v1/mms/upload")]
async fn upload_blob(
    MultipartForm(form): MultipartForm<UploadForm>,
    multimedia_management_service: web::Data<
        services::mutimedia_management_service::MutimediaManagementService,
    >,
) -> Result<HttpResponse, actix_web::Error> {
    for f in form.files {
        // create ./tmp required for file uploads
        if let Err(err) = fs::create_dir_all("./tmp") {
            log::error!("Failed to create directory: {}", err);
        }

        let file_name = f.file_name.unwrap();
        let mut current_dir_str = String::from("");
        if let Ok(current_dir) = std::env::current_dir() {
            current_dir_str = current_dir.to_string_lossy().to_string();
        }
        let modified_current_dir_str = current_dir_str.replace("\\", "/");
        let path = format!("{}/tmp/{}", modified_current_dir_str, file_name);
        let path_clone = path.clone();
        let path_clone_clone = path_clone.clone();
        log::info!("saving to {}", &path);
        f.file.persist(path).unwrap();

        let mut upload_file_parameters = services::upload_parameters::UploadFileParameters::new();
        upload_file_parameters.file_name = path_clone;
        upload_file_parameters.blob_name = file_name.clone();

        // Some mock data
        let mut upload_meta_parameters = services::upload_parameters::UploadMetaParameters::new();
        upload_meta_parameters.title = String::from("Peace");
        upload_meta_parameters.description = String::from("Peace for the world");
        upload_meta_parameters.tags = vec![Some(String::from("Nature"))];

        let result = multimedia_management_service
            .upload_blob_from_file_and_create_metadata(
                &upload_file_parameters,
                &upload_meta_parameters,
            )
            .await;

        if let Err(err) = result {
            if let Err(delete_error) = std::fs::remove_file(&path_clone_clone) {
                log::error!("Failed to delete the temporary file: {}", delete_error);
            }
            return Ok(HttpResponse::BadRequest().finish());
        } else {
            if let Err(delete_error) = std::fs::remove_file(&path_clone_clone) {
                log::error!("Failed to delete the temporary file: {}", delete_error);
            }
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

    let multi_media_management_service =
        services::mutimedia_management_service::MutimediaManagementService::new().await;

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
