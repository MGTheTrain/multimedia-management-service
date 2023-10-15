pub mod model;
pub mod container_meta;
pub mod track;
pub mod schema;

pub enum ModelType {
    ContainerMeta(container_meta::ContainerMeta),
    VideoTrack(track::VideoTrack),
    AudioTrack(track::AudioTrack),
    SubtitleTrack(track::SubtitleTrack),s
}
