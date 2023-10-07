// // NOTE: async traits not supported for now. See: https://stackoverflow.com/questions/65921581/how-can-i-define-an-async-method-in-a-trait
// use std::error::Error;

// pub trait BlobStorageConnector {
//     fn upload_blob(file_path: &str) -> Result<(), Box<dyn Error>>;
//     fn download_blob(file_path: &str) -> Result<(), Box<dyn Error>>;
//     fn delete_blob() -> Result<(), Box<dyn Error>>;
// }