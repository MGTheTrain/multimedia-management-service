use diesel_derive_enum::DbEnum;

#[derive(Debug, PartialEq, DbEnum)]
pub enum FileMetaType {
    Invalid,
    Video,
    Audio,
    Subtitle,
}
