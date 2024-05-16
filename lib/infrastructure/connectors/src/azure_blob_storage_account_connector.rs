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


use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
};

use azure_core::Error;
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use bytes::Bytes;
use log::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct AzureBlobStorageAccountConnector {
    container_client: Option<ContainerClient>,
}

impl AzureBlobStorageAccountConnector {
    /// Method new for constructing an object from the struct AzureBlobStorageAccountConnector
    ///
    /// This method takes no parameter,
    /// and returns an AzureBlobStorageAccountConnector object
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {    
        let azure_access_key =
            std::env::var("AZURE_ACCESS_KEY").expect("AZURE_ACCESS_KEY environment variable expected");
        let azure_account_name =
            std::env::var("AZURE_ACCOUNT_NAME").expect("AZURE_ACCOUNT_NAME environment variable expected");
        let azure_container_name =
            std::env::var("AZURE_CONTAINER_NAME").expect("AZURE_CONTAINER_NAME environment variable expected");
        let storage_credentials = StorageCredentials::access_key(azure_account_name.clone(), azure_access_key);
        Ok(AzureBlobStorageAccountConnector {
            container_client: Some(
                ClientBuilder::new(azure_account_name, storage_credentials)
                    .container_client(azure_container_name),
            ),
        })
    }

    /// Private method for returning a blob client
    ///
    /// This method takes &self and the blob_name,
    /// and returns an BlobClient object
    pub(crate) fn get_blob_client(&self, blob_name: &str) -> Option<BlobClient> {
        let blob_client = self
            .container_client
            .as_ref()
            .unwrap()
            .blob_client(blob_name);
        Some(blob_client)
    }

    /// Async method for uploading blobs to an Azure Storage Account Container
    ///
    /// This method takes &self, the blob_name and a file_path as parameters,
    /// and returns an Result<(), Error> object
    pub async fn upload_blob(&self, blob_name: &str, file_path: &str) -> Result<(), Error> {
        let blob_client = self.get_blob_client(blob_name).unwrap();
        let f = File::open(file_path)?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        blob_client.put_block_blob(buffer).await?;
        info!("Successfully uploaded blob {}", blob_name);
        Ok(())
    }

    /// Async method for retrieving the content of a blob from an Azure Storage Account Container
    ///
    /// This method takes &self and the blob_name as parameters,
    /// and returns an Result<Vec<u8>, Error> object
    pub async fn retrieve_bytes(&self, blob_name: &str) -> Result<Vec<u8>, Error> {
        let blob_client = self.get_blob_client(blob_name).unwrap();
        let data = blob_client.get_content().await?;
        Ok(data)
    }

    /// Async method for downloading blobs from an Azure Storage Account Container
    ///
    /// This method takes &self, the blob_name and a file_path as parameters,
    /// and returns an Result<(), Error> object
    pub async fn download_blob(&self, blob_name: &str, file_path: &str) -> Result<(), Error> {
        let data = self.retrieve_bytes(blob_name).await?;

        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)?;

        file.write_all(&data)?;
        info!("Successfully downloaded blob {}", blob_name);
        Ok(())
    }

    /// Async method for deleting blobs from an Azure Storage Account Container
    ///
    /// This method takes &self and the blob_name as parameters,
    /// and returns an Result<(), Error> object
    pub async fn delete_blob(&self, blob_name: &str) -> Result<(), Error> {
        let blob_client = self.get_blob_client(blob_name).unwrap();
        blob_client.delete().await?;
        info!("Successfully deleted blob {}", blob_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    async fn test_azure_storage_account_container_connector_methods() -> Result<(), Box<dyn std::error::Error>> {
        env_logger::init();

        let env_file_path = "./assets/az-secrets.dev.cfg";
        dotenv::from_path(env_file_path).ok();

        let azure_blob_storage_account_connector = 
            Box::new(AzureBlobStorageAccountConnector::new().unwrap());

        let upload_file_path = "./assets/sample.txt";
        let download_file_path = "./temp/sample-azure-copy.txt";
        let uuid = Uuid::new_v4();
        let blob_name = uuid.to_string() + "/sample.txt";
        let upload_blob_result = azure_blob_storage_account_connector
        .upload_blob(&blob_name, upload_file_path)
        .await;
        assert!(upload_blob_result.is_ok());
        let download_blob_result = azure_blob_storage_account_connector
            .download_blob(&blob_name, download_file_path)
            .await;
        assert!(download_blob_result.is_ok());
        let delete_blob_result = azure_blob_storage_account_connector
            .delete_blob(&blob_name)
            .await;
        assert!(delete_blob_result.is_ok());
        Ok(())
    }
}
