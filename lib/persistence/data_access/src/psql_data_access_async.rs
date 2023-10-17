// Samples:
// - https://github.com/weiznich/diesel_async/blob/main/tests/lib.rs 
// - https://github.com/tokio-rs/axum/blob/main/examples/diesel-async-postgres/src/main.rs 

use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncConnection, AsyncPgConnection};
use diesel::{
    Connection, PgConnection
};
use diesel_async::pooled_connection::{bb8::Pool, AsyncDieselConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use models::{
    container_meta::ContainerMeta,
    track::{self, VideoTrack, AudioTrack, SubtitleTrack},
};
use uuid::Uuid;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../domain/models/migrations");

#[derive(Clone)]
pub struct PsqlDataAccess {
    pub connection_pool: bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

impl PsqlDataAccess {
    /// Method for creating the PsqlDataAccess constructor
    ///
    /// Requires no parameters and returns and PsqlDataAccess object
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .cfg");
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
        let connection_pool = Pool::builder().build(config).await.unwrap();

        // migrations at compile time     
        // NOTE: workaround for async_diesel     
        let migration_database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .cfg");
        let mut migration_pg_connection = PgConnection::establish(&migration_database_url).
            unwrap_or_else(|_| panic!("Error connecting to {}", migration_database_url));
        info!("About to migrate datbase tables");
        migration_pg_connection.run_pending_migrations(MIGRATIONS).unwrap();

        Ok(PsqlDataAccess {
            connection_pool: connection_pool
        })
    }

    /// Method for inserting video_track rows into a Psql database table utilizing diesel ORM
    ///
    /// Requires an video_track as parameters and returns a Result<models::track::VideoTrack, diesel::result::Error>
    pub async fn insert_video_track(
        &self,
        video_track: &models::track::VideoTrack,
    ) -> Result<models::track::VideoTrack, diesel::result::Error> {
        use models::schema::video_track;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = diesel::insert_into(video_track::table)
        .values(video_track)
        .returning(models::track::VideoTrack::as_returning())
        .get_result(&mut pg_connection).await?;
        
        info!(
            "Successfully inserted video track id {}",
            video_track.id
        );
        Ok(result)
    }

    /// Method for inserting audio_track rows into a Psql database table utilizing diesel ORM
    ///
    /// Requires an audio_track as parameters and returns a Result<models::track::AudioTrack, diesel::result::Error>
    pub async fn insert_audio_track(
        &self,
        audio_track: &models::track::AudioTrack,
    ) -> Result<models::track::AudioTrack, diesel::result::Error> {
        use models::schema::audio_track;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = diesel::insert_into(audio_track::table)
        .values(audio_track)
        .returning(models::track::AudioTrack::as_returning())
        .get_result(&mut pg_connection).await?;
        
        info!(
            "Successfully inserted audio track id {}",
            audio_track.id
        );
        Ok(result)
    }

    /// Method for inserting subtitle_track rows into a Psql database table utilizing diesel ORM
    ///
    /// Requires an subtitle_track as parameters and returns a Result<models::track::SubtitleTrack, diesel::result::Error>
    pub async fn insert_subtitlte_track(
        &self,
        subtitle_track: &models::track::SubtitleTrack,
    ) -> Result<models::track::SubtitleTrack, diesel::result::Error> {
        use models::schema::subtitle_track;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = diesel::insert_into(subtitle_track::table)
        .values(subtitle_track)
        .returning(models::track::SubtitleTrack::as_returning())
        .get_result(&mut pg_connection).await?;
        
        info!(
            "Successfully inserted subtitle track id {}",
            subtitle_track.id
        );
        Ok(result)
    }

    /// Method for inserting container_meta rows into a Psql database table utilizing diesel ORM
    ///
    /// Requires an in_container_meta as parameters and returns a Result<models::container_meta::ContainerMeta, diesel::result::Error>
    pub async fn insert_container_meta(
        &self,
        in_container_meta: &models::container_meta::ContainerMeta,
    ) -> Result<models::container_meta::ContainerMeta, diesel::result::Error> {
        use models::schema::container_meta;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = diesel::insert_into(container_meta::table)
        .values(in_container_meta)
        .returning(models::container_meta::ContainerMeta::as_returning())
        .get_result(&mut pg_connection).await?;

        info!(
            "Successfully inserted container metainformation with container_meta_id {}",
            in_container_meta.id
        );
        Ok(result)
    }

    /// Method for retrieving a video_track row by id from a Psql database table utilizing diesel ORM
    ///
    /// Requires an in_track_id as parameters and returns a Result<models::track::VideoTrack, diesel::result::Error>
    pub async fn get_video_track_by_id(
        &self,
        track_id: &Uuid,
    ) -> Result<models::track::VideoTrack, diesel::result::Error> {
        use models::schema::video_track::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = video_track
            .filter(id.eq(track_id))
            .first::<models::track::VideoTrack>(&mut pg_connection).await?;

        info!(
            "Successfully retrieved video track by track id {}",
            track_id
        );
        Ok(result)
    }

    /// Method for retrieving a audio_track row by id from a Psql database table utilizing diesel ORM
    ///
    /// Requires an in_track_id as parameters and returns a Result<models::track::AudioTrack, diesel::result::Error>
    pub async fn get_audio_track_by_id(
        &self,
        track_id: &Uuid,
    ) -> Result<models::track::AudioTrack, diesel::result::Error> {
        use models::schema::audio_track::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = audio_track
            .filter(id.eq(track_id))
            .first::<models::track::AudioTrack>(&mut pg_connection).await?;

        info!(
            "Successfully retrieved audio track by track id {}",
            track_id
        );
        Ok(result)
    }

    /// Method for retrieving a audio_track row by id from a Psql database table utilizing diesel ORM
    ///
    /// Requires an in_track_id as parameters and returns a Result<models::track::AudioTrack, diesel::result::Error>
    pub async fn get_subtitle_track_by_id(
        &self,
        track_id: &Uuid,
    ) -> Result<models::track::SubtitleTrack, diesel::result::Error> {
        use models::schema::subtitle_track::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = subtitle_track
            .filter(id.eq(track_id))
            .first::<models::track::SubtitleTrack>(&mut pg_connection).await?;

        info!(
            "Successfully retrieved subtitle track by track id {}",
            track_id
        );
        Ok(result)
    }

    /// Method for retrieving a container_meta row by id from a Psql database table utilizing diesel ORM
    ///
    /// Requires a pg_connection, an in_container_meta_id as parameters and returns a Result<models::container_meta::ContainerMeta, diesel::result::Error>
    pub async fn get_container_meta_by_id(
        &self,
        container_meta_id: &Uuid,
    ) -> Result<models::container_meta::ContainerMeta, diesel::result::Error> {
        use models::schema::container_meta::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = container_meta
            .filter(id.eq(container_meta_id))
            .first::<models::container_meta::ContainerMeta>(&mut pg_connection).await?;

        info!(
            "Successfully retrieved container metainformation by container_meta_id {}",
            container_meta_id
        );
        Ok(result)
    }

    /// Method for updating a video_track row by id in a Psql database table utilizing diesel ORM
    ///
    /// Requires a track_id, an in_track_id as parameters and returns a Result<models::track::VideoTrack, diesel::result::Error>
    pub async fn update_video_track_by_id(
        &self,
        track_id: &Uuid,
        in_track: &models::track::VideoTrack,
    ) -> Result<models::track::VideoTrack, diesel::result::Error> {
        use models::schema::video_track::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = diesel::update(video_track.filter(id.eq(track_id)))
            .set((
                name.eq(&in_track.name),
                media_type.eq(&in_track.media_type),
                width.eq(&in_track.width),
                height.eq(&in_track.height),
                bit_rate.eq(&in_track.bit_rate),
                frame_rate.eq(&in_track.frame_rate),
            ))
            .returning(models::track::VideoTrack::as_returning())
            .get_result(&mut pg_connection).await?;

        info!(
            "Successfully updated video track by track_id {}",
            track_id
        );
        Ok(result)
    }

    /// Method for updating a audio_track row by id in a Psql database table utilizing diesel ORM
    ///
    /// Requires a track_id, an in_track_id as parameters and returns a Result<models::track::AudioTrack, diesel::result::Error>
    pub async fn update_audio_track_by_id(
        &self,
        track_id: &Uuid,
        in_track: &models::track::AudioTrack,
    ) -> Result<models::track::AudioTrack, diesel::result::Error> {
        use models::schema::audio_track::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = diesel::update(audio_track.filter(id.eq(track_id)))
            .set((
                name.eq(&in_track.name),
                media_type.eq(&in_track.media_type),
                bit_rate.eq(&in_track.bit_rate),
                channel_config.eq(&in_track.channel_config),
                sample_frequenz.eq(&in_track.sample_frequenz),
            ))
            .returning(models::track::AudioTrack::as_returning())
            .get_result(&mut pg_connection).await?;

        info!(
            "Successfully updated audio track by track_id {}",
            track_id
        );
        Ok(result)
    }

    /// Method for updating a subtitle_track row by id in a Psql database table utilizing diesel ORM
    ///
    /// Requires a track_id, an in_track_id as parameters and returns a Result<models::track::SubtitleTrack, diesel::result::Error>
    pub async fn update_subtitle_track_by_id(
        &self,
        track_id: &Uuid,
        in_track: &models::track::SubtitleTrack,
    ) -> Result<models::track::SubtitleTrack, diesel::result::Error> {
        use models::schema::subtitle_track::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = diesel::update(subtitle_track.filter(id.eq(track_id)))
            .set((
                name.eq(&in_track.name),
                media_type.eq(&in_track.media_type),
            ))
            .returning(models::track::SubtitleTrack::as_returning())
            .get_result(&mut pg_connection).await?;

        info!(
            "Successfully updated subtitle track by track_id {}",
            track_id
        );
        Ok(result)
    }

    /// Method for updating a container_meta row by id in a Psql database table utilizing diesel ORM
    ///
    /// Requires a container_meta_id, an in_container_meta_id as parameters and returns a Result<models::container_meta::ContainerMeta, diesel::result::Error>
    pub async fn update_container_meta_by_id(
        &self,
        container_meta_id: &Uuid,
        in_container_meta: &models::container_meta::ContainerMeta,
    ) -> Result<models::container_meta::ContainerMeta, diesel::result::Error> {
        use models::schema::container_meta::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = diesel::update(container_meta.filter(id.eq(container_meta_id)))
            .set((
                date_time_created.eq(&in_container_meta.date_time_created),
                date_time_updated.eq(&in_container_meta.date_time_updated),
                title.eq(&in_container_meta.title),
                description.eq(&in_container_meta.description),
                tags.eq(&in_container_meta.tags),
                video_track_id.eq(&in_container_meta.video_track_id),
                audio_track_id.eq(&in_container_meta.audio_track_id),
                subtitle_track_id.eq(&in_container_meta.subtitle_track_id),
                file_size_in_kb.eq(&in_container_meta.file_size_in_kb),
                duration.eq(&in_container_meta.duration),
            ))
            .returning(models::container_meta::ContainerMeta::as_returning())
            .get_result(&mut pg_connection).await?;

        info!(
            "Successfully updated container metainformation by container_meta_id {}",
            container_meta_id
        );
        Ok(result)
    }

    /// Method for deleting a video_track row by id in a Psql database table utilizing diesel ORM
    ///
    /// Requires a track_id, an in_track_id as parameters and returns a Result<(), diesel::result::Error>
    pub async fn delete_video_track_by_id(
        &self,
        track_id: &Uuid,
    ) -> Result<(), diesel::result::Error> {
        use models::schema::video_track::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let rows_deleted =
            diesel::delete(video_track.filter(id.eq(track_id))).execute(&mut pg_connection).await?;

        info!("Successfully deleted a video track {}", track_id);
        Ok(())
    }

    /// Method for deleting a audio_track row by id in a Psql database table utilizing diesel ORM
    ///
    /// Requires a track_id, an in_track_id as parameters and returns a Result<(), diesel::result::Error>
    pub async fn delete_audio_track_by_id(
        &self,
        track_id: &Uuid,
    ) -> Result<(), diesel::result::Error> {
        use models::schema::audio_track::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let rows_deleted =
            diesel::delete(audio_track.filter(id.eq(track_id))).execute(&mut pg_connection).await?;

        info!("Successfully deleted a audio track {}", track_id);
        Ok(())
    }

    /// Method for deleting a subtitle_track row by id in a Psql database table utilizing diesel ORM
    ///
    /// Requires a track_id, an in_track_id as parameters and returns a Result<(), diesel::result::Error>
    pub async fn delete_subtitle_track_by_id(
        &self,
        track_id: &Uuid,
    ) -> Result<(), diesel::result::Error> {
        use models::schema::subtitle_track::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let rows_deleted =
            diesel::delete(subtitle_track.filter(id.eq(track_id))).execute(&mut pg_connection).await?;

        info!("Successfully deleted a subtitle track {}", track_id);
        Ok(())
    }

    /// Method for deleting a container_meta row by id in a Psql database table utilizing diesel ORM
    ///
    /// Requires a container_meta_id, an in_container_meta_id as parameters and returns a Result<models::container_meta::ContainerMeta, diesel::result::Error>
    pub async fn delete_container_meta_by_id(
        &self,
        container_meta_id: &Uuid,
    ) -> Result<(), diesel::result::Error> {
        use models::schema::container_meta::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let rows_deleted = diesel::delete(container_meta.filter(id.eq(container_meta_id)))
            .execute(&mut pg_connection).await?;

        info!("Successfully deleted {}", container_meta_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use diesel::Identifiable;

    use super::*;

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    async fn test_psql_data_access_methods_for_track() -> Result<(), Box<dyn std::error::Error>>{
        env_logger::init();
        
        let env_file_path = "./assets/psql-secrets.dev.cfg";
        dotenv::from_path(env_file_path).ok();
        
        let psql_data_access = Box::new(PsqlDataAccess::new().await.unwrap());

        // file metainformation
        let mut video_track = VideoTrack {
            id: Uuid::new_v4(),
            container_meta_id: Uuid::new_v4(),
            name: String::from("simple_video.h264"),
            media_type: String::from("h264"),
            width: 1280,
            height: 720,
            bit_rate: 1850,
            frame_rate: 60,
        };

        // [C]reate
        let mut result = psql_data_access.insert_video_track(&video_track).await;
        assert!(result.is_ok());

        // [R]ead
        result = psql_data_access.get_video_track_by_id(&video_track.id).await;
        assert!(result.is_ok());
        // assert_eq!(video_track.name, result.as_ref().as_mut().name);
        // assert_eq!(video_track.container_meta_id, &result.container_meta_id);

        // [U]pdate
        video_track.name = String::from("simple_updated_video.h264");
        result = psql_data_access.update_video_track_by_id(
            &video_track.id,
            &video_track,
        ).await;
        assert!(result.is_ok());

        // [D]elete
        let mut delete_result =
            psql_data_access.delete_video_track_by_id(&video_track.id).await;
        assert!(result.is_ok());

        Ok(())
    }
}
