use std::{io::{BufReader, Read, Write}, fs::{File, self}};

use azure_core::Error;
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;

pub struct AzureBlobStorageAccountConnector {
    container_client: Option<ContainerClient>,
}

impl AzureBlobStorageAccountConnector {
    /// Method new for constructing an object from the struct AzureBlobStorageAccountConnector
    ///
    /// This method takes the account_name, access_key and container_name parameters,
    /// and returns an AzureBlobStorageAccountConnector object
    fn new(
        account_name: &str, 
        access_key: &str,
        container_name: &str) -> Self {
        let storage_credentials = StorageCredentials::access_key(account_name.clone(), access_key);
        AzureBlobStorageAccountConnector {
            container_client: Some(ClientBuilder::new(account_name, storage_credentials).container_client(container_name)),
        }
    }

    /// Private helper method for returning a blob client
    /// 
    /// This method takes &self and the blob_name,
    /// and returns an BlobClient object
    pub(crate) fn get_blob_client(&self, blob_name: &str) -> BlobClient {
        let blob_client = self.container_client.as_ref().unwrap().blob_client(blob_name);
        blob_client
    }

    /// Method for uploading blobs to a Storage Account Container
    ///
    /// This method takes &self, the blob_name and an file_path,
    /// and returns an Result object
    async fn upload_blob(&self, blob_name: &str, file_path: &str) -> Result<(), Error> {
        let blob_client = self.get_blob_client(blob_name);
        let f = File::open(file_path)?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        blob_client.put_block_blob(buffer).await?;
        Ok(())
    }

    /// Method for downloading blobs to a Storage Account Container
    ///
    /// This method takes &self, the blob_name and an file_path,
    /// and returns an Result object
    async fn download_blob(&self, blob_name: &str, file_path: &str) -> Result<(), Error> {
        let blob_client = self.get_blob_client(blob_name);
        let data = blob_client.get_content().await?;

        let mut file = fs::OpenOptions::new()
            .create(true) 
            .write(true)
            .open(file_path)?;
    
        file.write_all(&data)?;
        Ok(())
    }

    async fn delete_blob(&self, blob_name: &str) -> Result<(), Box<Error>> {
        let blob_client = self.get_blob_client(blob_name);
        blob_client.delete().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_azure_storage_account_container_connector_methods() {
        let azure_blob_storage_account_connector = 
            Box::new(AzureBlobStorageAccountConnector::new("","",""));  
        

    }
}
