#[derive(Debug, PartialEq)]
pub enum FileMetaType {
    Invalid,
    Video,
    Audio,
    Subtitle,
}

impl FileMetaType {
    pub fn to_i32(&self) -> i32 {
        match self {
            FileMetaType::Invalid => 0,
            FileMetaType::Video => 1,
            FileMetaType::Audio => 2,
            FileMetaType::Subtitle => 3,
        }
    }
}
