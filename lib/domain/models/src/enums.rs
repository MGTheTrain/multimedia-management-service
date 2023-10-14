#[derive(Debug, PartialEq)]
pub enum TrackType {
    Invalid,
    Video,
    Audio,
    Subtitle,
}

impl TrackType {
    pub fn to_i32(&self) -> i32 {
        match self {
            TrackType::Invalid => 0,
            TrackType::Video => 1,
            TrackType::Audio => 2,
            TrackType::Subtitle => 3,
        }
    }
}
