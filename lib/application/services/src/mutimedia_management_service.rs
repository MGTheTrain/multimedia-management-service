extern crate connectors;
extern crate data_access;

struct MutimediaManagementService {
    connector: connectors::aws_s3_bucket_connector::AwsS3BucketConnector,
    data_access: data_access::psql_data_access_async::PsqlDataAccess,
}

impl MutimediaManagementService {
    /// Method for creating the MutimediaManagementService constructor
    ///
    /// Requires no parameters and returns and MutimediaManagementService object
    async fn new() -> Self {
        MutimediaManagementService {
            connector: connectors::aws_s3_bucket_connector::AwsS3BucketConnector::new().await,
            data_access: data_access::psql_data_access_async::PsqlDataAccess::new(),
        }
    }

    /// Method for uploading blobs to a blob storage and inserting for the uploaded file file_meta rows into a Psql database table utilizing diesel ORM
    ///
    /// Requires a bucket_name, a key, a file_name and returns a Result<models::file_meta::FileMeta, diesel::result::Error>
    async fn upload_blob(        
        bucket_name: &str,
        key: &str,
        file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
            


        Ok(())
    }    

    async fn download_blob_by_id() -> Result<(), Box<dyn std::error::Error>> {

        Ok(())
    }    
}