use aws_sdk_s3::{Client, Error, operation::{create_bucket::{CreateBucketOutput, CreateBucketError}, put_object::{PutObjectOutput, PutObjectError}, get_object::{GetObjectOutput, GetObjectError}}, error::SdkError, types::{BucketLocationConstraint, CreateBucketConfiguration}, primitives::ByteStream};
use std::{io::{Write, self}, env, path::Path}; // bring trait into scope
use std::fs;

pub struct AwsS3BucketConnector {
    storage_client: Option<Client>,
}

impl AwsS3BucketConnector {
    /// Method new for constructing an object from the struct AwsS3BucketConnector
    ///
    /// This method takes no parameters,
    /// and returns an AwsS3BucketConnector object
    fn new() -> Self {
        let config = aws_config::load_from_env();
        AwsS3BucketConnector {
            client = Some(Client::new(&config)),
        }
    }

    /// Private method for returning a blob client
    /// 
    /// This method takes &self and the blob_name as parameters,
    /// and returns an BlobClient object
    pub(crate) fn get_blob_client(&self, blob_name: &str) -> Option<BlobClient> {
        let blob_client = self.storage_client.as_ref().unwrap().blob_client(blob_name);
        Some(blob_client)
    }

    /// Async method for uploading blobs to an AWS S3 Bucket
    ///
    /// This method takes &self, the blob_name and a file_path as parameters,
    /// and returns an Result<(), Error> object
    async fn upload_blob(&self, blob_name: &str, file_path: &str) -> Result<(), Error> {

        info!("Successfully uploaded blob {}", blob_name);
        Ok(())
    }


    /// Async method for retrieving the content of a blob from an AWS S3 Bucket
    ///
    /// This method takes &self and the blob_name as parameters,
    /// and returns an Result<Vec<u8>, Error> object
    async fn retrieve_bytes(&self, blob_name: &str) -> Result<Vec<u8>, Error> {
        
        Ok(data)
    }

    /// Async method for downloading blobs from an AWS S3 Bucket
    ///
    /// This method takes &self, the blob_name and a file_path as parameters,
    /// and returns an Result<(), Error> object
    async fn download_blob(&self, blob_name: &str, file_path: &str) -> Result<(), Error> {
        
        info!("Successfully downloaded blob {}", blob_name);
        Ok(())
    }

    /// Async method for deleting blobs from an AWS S3 Bucket
    ///
    /// This method takes &self and the blob_name as parameters,
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

        let azure_blob_storage_account_connector = 
            Box::new(AwsS3BucketConnector::new(
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
