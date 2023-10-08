use crate::schema::file_meta;
use diesel::prelude::*;
use uuid::Uuid;

use crate::enums::FileMetaType;
use crate::model::Model;

#[derive(Insertable, Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = file_meta)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FileMeta {
    pub id: Uuid,
    pub container_id: Uuid,
    pub name: String,
    pub file_type: i32, /// utilize FileMetaType enum for file_type
    pub file_size_in_kb: i64,
}

impl Model for FileMeta {
    fn new() -> Self {
        FileMeta {
            id: Uuid::new_v4(),
            container_id: Uuid::new_v4(),
            name: String::from(""),
            file_type: 0, 
            file_size_in_kb: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_meta() {
        // FileMeta::new() when wanting to create a file meta object trough its constructor on the stack memory
        let mut file_meta_type = FileMetaType::Video;
        let video_file_meta = Box::new(FileMeta {
            id: Uuid::new_v4(),
            container_id: Uuid::new_v4(),
            name: String::from("simple_video.h264"),
            file_type: file_meta_type.to_i32(),
            file_size_in_kb: 200000,
        });
        assert_eq!(video_file_meta.id, 1);
        assert_eq!(video_file_meta.name, String::from("simple_video.h264"));
        assert_eq!(video_file_meta.file_type, FileMetaType::Video.to_i32());
        assert_eq!(video_file_meta.file_size_in_kb, 200000);

        file_meta_type = FileMetaType::Audio;
        let audio_file_meta = Box::new(FileMeta {
            id: Uuid::new_v4(),
            container_id: Uuid::new_v4(),
            name: String::from("simple_audio.aac"),
            file_type: file_meta_type.to_i32(),
            file_size_in_kb: 150000,
        });
        assert_eq!(audio_file_meta.id, 2);
        assert_eq!(audio_file_meta.name, String::from("simple_audio.aac"));
        assert_eq!(audio_file_meta.file_type, FileMetaType::Audio.to_i32());
        assert_eq!(audio_file_meta.file_size_in_kb, 150000);
    }
}
