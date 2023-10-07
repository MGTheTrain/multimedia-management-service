use aws_sdk_s3::{Client, Error, operation::{create_bucket::{CreateBucketOutput, CreateBucketError}, put_object::{PutObjectOutput, PutObjectError}, get_object::{GetObjectOutput, GetObjectError}}, error::SdkError, types::{BucketLocationConstraint, CreateBucketConfiguration}, primitives::ByteStream};
use std::{io::{Write, self}, env, path::Path}; // bring trait into scope
use std::fs;
use bytes::Bytes;

use log::{info};

pub struct AwsS3BucketConnector {
    storage_client: Option<Client>,
}

impl AwsS3BucketConnector {
    /// Method new for constructing an object from the struct AwsS3BucketConnector
    ///
    /// This method takes no parameters,
    /// and returns an AwsS3BucketConnector object
    async fn new() -> Self {
        let config = aws_config::load_from_env().await;
        AwsS3BucketConnector {
            storage_client: Some(Client::new(&config)),
        }
    }

    /// Private method for returning a blob client
    /// 
    /// This method takes &self, the bucket name and a key as parameters,
    /// and returns an Result<GetObjectOutput, SdkError<GetObjectError>> object
    async fn get_object(
        &self, bucket_name: &str, key: &str) -> Result<GetObjectOutput, SdkError<GetObjectError>> {
        self.storage_client
            .as_ref()
            .unwrap()
            .get_object()
            .bucket(bucket_name)
            .key(key)
            .send()
            .await
    }

    /// Async method for uploading blobs to an AWS S3 Bucket
    ///
    /// This method takes &self, the bucket_name, the key (alias blob_name) and the file_name  as parameters,
    /// and returns an Result<PutObjectOutput, SdkError<PutObjectError>> object
    async fn upload_blob(
        &self, 
        bucket_name: &str,
        key: &str,
        file_name: &str) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
        let body = ByteStream::from_path(Path::new(file_name)).await;
        let put_object_output = self.storage_client
            .as_ref()
            .unwrap()
            .put_object()
            .bucket(bucket_name)
            .key(key)
            .body(body.unwrap())
            .send()
            .await;
        info!("Successfully uploaded blob {}", key);
        put_object_output
    }

    /// Async method for writing the blobs content/bytes from an AWS S3 Bucket to a file
    ///
    /// This method takes &self, the bucket_name, the key and a file_path as parameters,
    /// and returns an Result<(), Error> object
    async fn write_bytes_to_file(
        &self,
        bytes: &Bytes,
        file_path: &str,) -> Result<(), io::Error> {      
        let mut file = fs::OpenOptions::new()
            .create(true) // To create a new file
            .write(true)
            .open(file_path)?;

        file.write_all(&bytes)?;
        info!("Successfully created file {}", file_path);
        Ok(())
    }

    /// Async method for deleting blobs from an AWS S3 Bucket
    ///
    /// This method takes &self, the bucket_name and the key as parameters,
    /// and returns an Result<(), Error> object
    async fn delete_blob(&self, bucket_name: &str, key: &str) -> Result<(), Error> {
        self.storage_client
            .as_ref()
            .unwrap()
            .delete_object()
            .bucket(bucket_name)
            .key(key)
            .send()
            .await?;

        info!("Successfully deleted blob {}", key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    async fn test_aws_s3_bucket_connector_methods() -> Result<(), Error> {
        env_logger::init();

        let env_file_path = "./assets/aws-secrets.dev.cfg";
        dotenv::from_path(env_file_path).ok();

        let aws_s3_bucket_connector = Box::new(AwsS3BucketConnector::new().await);  

        let bucket_name = "mg-test-bucket-v25";     
        let key = "sample.txt";
        let upload_file_path = "assets/sample.txt";
        let download_file_path = "temp/sample-aws-copy.txt";
        let upload_blob_result = aws_s3_bucket_connector.upload_blob(bucket_name, key, upload_file_path).await;
        assert!(upload_blob_result.is_ok());
        let get_object_output = aws_s3_bucket_connector.get_object(bucket_name, key).await;
        assert!(get_object_output.is_ok());
        let bytes = get_object_output?.body.collect().await.unwrap().into_bytes();
        assert!(bytes.len() > 0);
        let write_bytes_to_file_result = aws_s3_bucket_connector.write_bytes_to_file(&bytes, download_file_path).await;
        assert!(write_bytes_to_file_result.is_ok());
        let delete_blob_result = aws_s3_bucket_connector.delete_blob(bucket_name, key).await;
        assert!(delete_blob_result.is_ok());
        Ok(())
    }
}
