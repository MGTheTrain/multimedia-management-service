extern crate models;

pub trait IVideoMnagementService {
    fn upload_multi_media_container(&Self, ) -> models::container_meta::ContainerMeta; 
    fn download_multi_media_container(&self, id: i32) -> models::container_meta::ContainerMeta; 
    fn retrieve_multi_media_container_metainformations(
        &self, 
        date_time_created: DateTime<Utc>,  
        date_time_updated: DateTime<Utc>, 
        title: String, 
        description: String, 
        tags: Vec<String>) -> Vec<models::container_meta::ContainerMeta>; 
    fn retrieve_multi_media_container_metainformation_by_id(&self, id: i32) -> models::container_meta::ContainerMeta; 
    fn update_multi_media_container_metainformation_by_id(&self, id: i32, container_meta: models::container_meta::ContainerMeta) -> models::container_meta::ContainerMeta; 
    fn delete_multi_media_container_metainformation_by_id(&self, id: i32) -> models::container_meta::ContainerMeta; 
}