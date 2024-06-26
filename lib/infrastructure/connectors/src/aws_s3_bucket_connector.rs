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


use aws_sdk_s3::{
    error::SdkError,
    operation::{
        create_bucket::{CreateBucketError, CreateBucketOutput},
        get_object::{GetObjectError, GetObjectOutput},
        put_object::{PutObjectError, PutObjectOutput},
    },
    primitives::ByteStream,
    types::{BucketLocationConstraint, CreateBucketConfiguration},
    Client, Error,
};
use bytes::Bytes;
use std::fs;
use std::{
    env,
    io::{self, Write},
    path::Path,
};

use log::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct AwsS3BucketConnector {
    bucket_name: Option<String>,
    storage_client: Option<Client>,
}

impl AwsS3BucketConnector {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID environment variable expected");
        std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY environment variable expected");
        std::env::var("AWS_DEFAULT_REGION").expect("AWS_DEFAULT_REGION environment variable expected");
        std::env::var("AWS_ENDPOINT_URL").expect("AWS_ENDPOINT_URL environment variable expected");
        let bucket_name =
            std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME environment variable expected");

        let config = aws_config::load_from_env().await;
        Ok(AwsS3BucketConnector {
            bucket_name: Some(String::from(bucket_name)),
            storage_client: Some(Client::new(&config)),
        })
    }

    pub async fn get_object(
        &self,
        blob_name: &str,
    ) -> Result<GetObjectOutput, SdkError<GetObjectError>> {
        self.storage_client
            .as_ref()
            .unwrap()
            .get_object()
            .bucket(self.bucket_name.as_ref().unwrap())
            .key(blob_name)
            .send()
            .await
    }

    pub async fn upload_blob(
        &self,
        blob_name: &str,
        file_name: &str,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
        let body = ByteStream::from_path(Path::new(file_name)).await;
        let put_object_output = self
            .storage_client
            .as_ref()
            .unwrap()
            .put_object()
            .bucket(self.bucket_name.as_ref().unwrap())
            .key(blob_name)
            .body(body.unwrap())
            .send()
            .await;
        info!("Successfully uploaded blob {}", blob_name);
        put_object_output
    }

    pub async fn upload_bytes(
        &self,
        blob_name: &str,
        bytes: Vec<u8>,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
        let body = ByteStream::from(bytes);
        let put_object_output = self
            .storage_client
            .as_ref()
            .unwrap()
            .put_object()
            .bucket(self.bucket_name.as_ref().unwrap())
            .key(blob_name)
            .body(body)
            .send()
            .await;
        info!("Successfully uploaded blob {}", blob_name);
        put_object_output
    }

    pub async fn write_bytes_to_file(&self, bytes: &Bytes, file_path: &str) -> Result<(), io::Error> {
        let mut file = fs::OpenOptions::new()
            .create(true) // To create a new file
            .write(true)
            .open(file_path)?;

        file.write_all(&bytes)?;
        info!("Successfully created file {}", file_path);
        Ok(())
    }

    pub async fn delete_blob(&self, blob_name: &str) -> Result<(), Error> {
        self.storage_client
            .as_ref()
            .unwrap()
            .delete_object()
            .bucket(self.bucket_name.as_ref().unwrap())
            .key(blob_name)
            .send()
            .await?;

        info!("Successfully deleted blob {}", blob_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    async fn test_aws_s3_bucket_connector_methods() -> Result<(), Box<dyn std::error::Error>> {
        env_logger::init();

        let env_file_path = "./assets/aws-secrets.dev.cfg";
        dotenv::from_path(env_file_path).ok();
        let aws_s3_bucket_connector = Box::new(AwsS3BucketConnector::new().await.unwrap());

        let upload_file_path = "assets/sample.txt";
        let download_file_path = "temp/sample-aws-copy.txt";
        let uuid = Uuid::new_v4();
        let blob_name = uuid.to_string() + "/sample.txt";
        let upload_blob_result = aws_s3_bucket_connector
            .upload_blob(&blob_name, upload_file_path)
            .await;
        assert!(upload_blob_result.is_ok());
        let get_object_output = aws_s3_bucket_connector.get_object(&blob_name).await;
        assert!(get_object_output.is_ok());
        let bytes = get_object_output?
            .body
            .collect()
            .await
            .unwrap()
            .into_bytes(); // retrieve bytes
        assert!(bytes.len() > 0);
        let write_bytes_to_file_result = aws_s3_bucket_connector
            .write_bytes_to_file(&bytes, download_file_path)
            .await;
        assert!(write_bytes_to_file_result.is_ok());
        let delete_blob_result = aws_s3_bucket_connector.delete_blob(&blob_name).await;
        assert!(delete_blob_result.is_ok());
        Ok(())
    }
}
