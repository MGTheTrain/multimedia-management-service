use uuid::Uuid;

pub struct UploadBlobParameters {
    pub blob_name: String,
    pub file_name: String,
}
pub struct UploadMetaParameters {
    pub title: String,
    pub description: String,
    pub tags: Vec<Option<String>>,
}