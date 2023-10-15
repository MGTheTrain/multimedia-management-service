use uuid::Uuid;

pub struct UploadFileParameters {
    pub blob_name: String,
    pub file_name: String,
}

impl UploadFileParameters {
    pub fn new() -> Self {
        UploadFileParameters {
            blob_name: String::from(""),
            file_name: String::from(""),
        }
    }
}

pub struct UploadBytesParameters {
    pub blob_name: String,
    pub file_name: String,
    pub bytes: Vec<u8>,
}

impl UploadBytesParameters {
    pub fn new() -> Self {
        UploadBytesParameters {
            blob_name: String::from(""),
            file_name: String::from(""),
            bytes: Vec::new(),
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