
impl AwsS3BucketConnector {
    /// Method new for constructing an object from the struct AzureBlobStorageAccountConnector
    ///
    /// This method takes the account_name, access_key and container_name parameters,
    /// and returns an AzureBlobStorageAccountConnector object
    fn new(
        account_name: &str, 
        access_key: &str,
        container_name: &str) -> Self {
        
    }

    /// Private method for returning a blob client
    /// 
    /// This method takes &self and the blob_name,
    /// and returns an BlobClient object
    pub(crate) fn get_blob_client(&self, blob_name: &str) -> Option<BlobClient> {
        let blob_client = self.container_client.as_ref().unwrap().blob_client(blob_name);
        Some(blob_client)
    }

    /// Async method for uploading blobs to an Azure Storage Account Container
    ///
    /// This method takes &self, the blob_name and an file_path,
    /// and returns an Result<(), Error> object
    async fn upload_blob(&self, blob_name: &str, file_path: &str) -> Result<(), Error> {

        info!("Successfully uploaded blob {}", blob_name);
        Ok(())
    }


    /// Async method for retrieving the content of a blob from an Azure Storage Account Container
    ///
    /// This method takes &self and the blob_name,
    /// and returns an Result<Vec<u8>, Error> object
    async fn retrieve_bytes(&self, blob_name: &str) -> Result<Vec<u8>, Error> {
        
        Ok(data)
    }

    /// Async method for downloading blobs from an Azure Storage Account Container
    ///
    /// This method takes &self, the blob_name and an file_path,
    /// and returns an Result<(), Error> object
    async fn download_blob(&self, blob_name: &str, file_path: &str) -> Result<(), Error> {
        
        info!("Successfully downloaded blob {}", blob_name);
        Ok(())
    }

    /// Async method for deleting blobs from an Azure Storage Account Container
    ///
    /// This method takes &self, the blob_name,
    /// and returns an Result<(), Error> object
    async fn delete_blob(&self, blob_name: &str) -> Result<(), Error> {
        
        info!("Successfully deleted blob {}", blob_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    async fn test_aws_s3_bucket_connector_methods() -> azure_core::Result<()> {
        env_logger::init();

        let env_file_path = "./assets/aws-secrets.dev.cfg";
        dotenv::from_path(env_file_path).ok();

        let azure_access_key = std::env::var("AZURE_ACCESS_KEY").expect("AZURE_ACCESS_KEY not found in .cfg");
        let azure_account_name = std::env::var("AZURE_ACCOUNT_NAME").expect("AZURE_ACCOUNT_NAME not found in .cfg");
        let azure_container_name = std::env::var("AZURE_CONTAINER_NAME").expect("AZURE_CONTAINER_NAME not found in .cfg");

        let azure_blob_storage_account_connector = 
            Box::new(AzureBlobStorageAccountConnector::new(
                azure_account_name.as_str(), azure_access_key.as_str(), azure_container_name.as_str()));  
        
        let upload_file_path = "./assets/sample.txt";
        let download_file_path = "./temp/copy-sample.txt";
        let blob_name = "sample.txt";
        assert_eq!(azure_blob_storage_account_connector.upload_blob(blob_name, upload_file_path).await?, ());
        assert_eq!(azure_blob_storage_account_connector.download_blob(blob_name, download_file_path).await?, ());
        assert_eq!(azure_blob_storage_account_connector.delete_blob(blob_name).await?, ());
        Ok(())
    }
}
