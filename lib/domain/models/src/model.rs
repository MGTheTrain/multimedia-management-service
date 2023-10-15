pub enum ModelType {
    ContainerMeta,
    VideoTrack,
    AudioTrack,
    SubtitleTrack
}

pub trait Model {
    fn new() -> Self;
}