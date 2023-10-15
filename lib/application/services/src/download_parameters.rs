pub struct DownloadBlobParameters {
    pub container_meta_id: String,
    pub file_name: String,
}

impl DownloadBlobParameters {
    pub fn new() -> Self {
        DownloadBlobParameters{
            container_meta_id: String::from(""),
            file_name: String::from(""),
        }
    }
}