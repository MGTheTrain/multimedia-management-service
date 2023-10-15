use uuid::Uuid;

pub struct UploadBlobParameters {
    pub blob_name: String,
    pub file_name: String,
}

impl UploadBlobParameters {
    pub fn new() -> Self {
        UploadBlobParameters {
            blob_name: String::from(""),
            file_name: String::from(""),
        }
    }
}
pub struct UploadMetaParameters {
    pub title: String,
    pub description: String,
    pub tags: Vec<Option<String>>,
}

impl UploadMetaParameters {
    pub fn new() -> Self {
        UploadMetaParameters { 
            title: String::from(""), 
            description: String::from(""), 
            tags: Vec::new(), 
        }
    }
}