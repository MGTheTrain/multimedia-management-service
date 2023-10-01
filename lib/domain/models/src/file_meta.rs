use crate::enums::FileMetaType;
use crate::model::Model;

pub struct FileMeta {
    pub id: i32,
    pub name: String,
    pub file_type: FileMetaType,
    pub file_size_in_kb: i64,
}

impl Model for FileMeta {
    fn new() -> Self {
        FileMeta {
            id: -1,
            name: String::from(""),
            file_type: FileMetaType::Invalid,
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
        let video_file_meta = Box::new(FileMeta {
            id: 1,
            name: String::from("simple_video.h264"),
            file_type: FileMetaType::Video,
            file_size_in_kb: 200000,
        });
        assert_eq!(video_file_meta.id, 1);
        assert_eq!(video_file_meta.name, String::from("simple_video.h264"));
        assert_eq!(video_file_meta.file_type, FileMetaType::Video);
        assert_eq!(video_file_meta.file_size_in_kb, 200000);

        let audio_file_meta = Box::new(FileMeta {
            id: 2,
            name: String::from("simple_audio.aac"),
            file_type: FileMetaType::Audio,
            file_size_in_kb: 150000,
        });
        assert_eq!(audio_file_meta.id, 2);
        assert_eq!(audio_file_meta.name, String::from("simple_audio.aac"));
        assert_eq!(audio_file_meta.file_type, FileMetaType::Audio);
        assert_eq!(audio_file_meta.file_size_in_kb, 150000);
    }
}
