use uuid::Uuid;

pub struct UploadBlobParameters {
    pub key: String,
    pub file_name: String,
}
pub struct UploadMetaParameters {
    pub title: String,
    pub description: String,
    pub tags: Vec<Option<String>>,
}