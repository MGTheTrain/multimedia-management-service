pub struct DeleteBlobParameters {
    pub container_meta_id: String,
    pub file_name: String,
}

impl DeleteBlobParameters {
    pub fn new() -> Self {
        DeleteBlobParameters{
            container_meta_id: String::from(""),
            file_name: String::from(""),
        }
    }

    pub fn get_blob_name(&self) -> String {
        String::from(&self.container_meta_id) + "/" + &self.file_name
    }
}