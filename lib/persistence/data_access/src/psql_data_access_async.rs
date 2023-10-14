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
use models::enums::TrackType;
use models::{
    container_meta::ContainerMeta,
    track::{self, Track},
};
use uuid::Uuid;
extern crate models;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../domain/models/migrations");

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

        Ok(PsqlDataAccess {
            connection_pool: connection_pool
        })
    }
    /// Method for inserting track rows into a Psql database table utilizing diesel ORM
    ///
    /// Requires an in_track as parameters and returns a Result<models::track::Track, diesel::result::Error>
    pub async fn insert_track(
        &self,
        in_track: &models::track::Track,
    ) -> Result<models::track::Track, diesel::result::Error> {
        use models::schema::track;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = diesel::insert_into(track::table)
        .values(in_track)
        .returning(models::track::Track::as_returning())
        .get_result(&mut pg_connection).await?;
        
        info!(
            "Successfully inserted file metainformation with track_id {}",
            in_track.id
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

    /// Method for retrieving a track row by id from a Psql database table utilizing diesel ORM
    ///
    /// Requires an in_track_id as parameters and returns a Result<models::track::Track, diesel::result::Error>
    pub async fn get_track_by_id(
        &self,
        track_id: &Uuid,
    ) -> Result<models::track::Track, diesel::result::Error> {
        use models::schema::track::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = track
            .filter(id.eq(track_id))
            .first::<models::track::Track>(&mut pg_connection).await?;

        info!(
            "Successfully retrieved file metainformation by track_id {}",
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

    /// Method for updating a track row by id in a Psql database table utilizing diesel ORM
    ///
    /// Requires a track_id, an in_track_id as parameters and returns a Result<models::track::Track, diesel::result::Error>
    pub async fn update_track_by_id(
        &self,
        track_id: &Uuid,
        in_track: &models::track::Track,
    ) -> Result<models::track::Track, diesel::result::Error> {
        use models::schema::track::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let result = diesel::update(track.filter(id.eq(track_id)))
            .set((
                name.eq(&in_track.name),
                file_type.eq(&in_track.file_type)
            ))
            .returning(models::track::Track::as_returning())
            .get_result(&mut pg_connection).await?;

        info!(
            "Successfully updated file metainformation by track_id {}",
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
                track_ids.eq(&in_container_meta.track_ids),
            ))
            .returning(models::container_meta::ContainerMeta::as_returning())
            .get_result(&mut pg_connection).await?;

        info!(
            "Successfully updated container metainformation by container_meta_id {}",
            container_meta_id
        );
        Ok(result)
    }

    /// Method for deleting a track row by id in a Psql database table utilizing diesel ORM
    ///
    /// Requires a track_id, an in_track_id as parameters and returns a Result<models::track::Track, diesel::result::Error>
    pub async fn delete_track_by_id(
        &self,
        track_id: &Uuid,
    ) -> Result<(), diesel::result::Error> {
        use models::schema::track::dsl::*;

        let mut pg_connection = self.connection_pool.get().await.unwrap();
        let rows_deleted =
            diesel::delete(track.filter(id.eq(track_id))).execute(&mut pg_connection).await?;

        info!("Successfully deleted {}", track_id);
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
        
        // migrations at compile time     
        // NOTE: workaround for async_diesel     
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .cfg");
        let mut migration_pg_connection = PgConnection::establish(&database_url).
            unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        info!("About to migrate datbase tables");
        migration_pg_connection.run_pending_migrations(MIGRATIONS).unwrap();

        // file metainformation
        let mut track_type = TrackType::Video;
        let mut video_track = Track {
            id: Uuid::new_v4(),
            container_meta_id: Uuid::new_v4(),
            name: String::from("simple_video.h264"),
            file_type: track_type.to_i32(),
        };

        track_type = TrackType::Audio;
        let mut audio_track = Box::new(Track {
            id: Uuid::new_v4(),
            container_meta_id: Uuid::new_v4(),
            name: String::from("simple_audio.aac"),
            file_type: track_type.to_i32(),
        });

        // [C]reate
        let mut result = psql_data_access.insert_track(&video_track).await;
        assert!(result.is_ok());
        result = psql_data_access.insert_track(&audio_track).await;
        assert!(result.is_ok());

        // [R]ead
        result = psql_data_access.get_track_by_id(&video_track.id).await;
        assert!(result.is_ok());
        // assert_eq!(video_track.name, result.as_ref().as_mut().name);
        // assert_eq!(video_track.container_meta_id, &result.container_meta_id);
        result = psql_data_access.get_track_by_id(&audio_track.id).await;
        assert!(result.is_ok());
        // assert_eq!(audio_track.name, &result.name);
        // assert_eq!(audio_track.container_meta_id, &result.container_meta_id);

        // [U]pdate
        video_track.name = String::from("simple_updated_video.h264");
        result = psql_data_access.update_track_by_id(
            &video_track.id,
            &video_track,
        ).await;
        assert!(result.is_ok());

        // [D]elete
        let mut delete_result =
            psql_data_access.delete_track_by_id(&video_track.id).await;
        assert!(result.is_ok());

        delete_result =
            psql_data_access.delete_track_by_id(&audio_track.id).await;
        assert!(result.is_ok());
        Ok(())
    }
}
