use diesel::{ConnectionResult, PgConnection, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, result::Error, SelectableHelper};
use models::{file_meta::{FileMeta, self}, container_meta::ContainerMeta};
extern crate models;

pub struct PsqlDataAccess {
    pg_connection: Option<PgConnection>,
}

impl PsqlDataAccess {
  /// Method new for constructing an object from the struct PsqlDataAccess
    ///
    /// This method takes the database_url as a parameter,
    /// and returns an PsqlDataAccess object
    async fn new(database_url: &str) -> Result<Self, diesel::ConnectionError> {
        let pg_connection = PgConnection::establish(database_url)?;
        Ok(PsqlDataAccess {
            pg_connection: Some(pg_connection),
        })
    }

    fn insert_file_meta(&self, pg_connection: &mut PgConnection, in_file_meta: &models::file_meta::FileMeta) -> Result<models::file_meta::FileMeta, diesel::result::Error> {
        use models::schema::file_meta;
        
        Ok(diesel::insert_into(file_meta::table)
        .values(in_file_meta)
        .returning(models::file_meta::FileMeta::as_returning())
        .get_result(pg_connection)
        .expect("Error saving file meta"))
    }

    fn insert_container_meta(&self, pg_connection: &mut PgConnection, in_container_meta: &models::container_meta::ContainerMeta) -> Result<models::container_meta::ContainerMeta, diesel::result::Error> {
        use models::schema::container_meta;
        
        Ok(diesel::insert_into(container_meta::table)
        .values(in_container_meta)
        .returning(models::container_meta::ContainerMeta::as_returning())
        .get_result(pg_connection)
        .expect("Error saving file meta"))
    }

    fn get_file_meta_by_id(&self, pg_connection: &mut PgConnection, file_meta_id: &i32) -> Result<models::file_meta::FileMeta, diesel::result::Error> {
        use models::schema::file_meta::dsl::*;
        
        let result = file_meta
        .filter(id.eq(file_meta_id))
        .first::<models::file_meta::FileMeta>(pg_connection)?;
        Ok(result)
    }
    

    fn get_container_meta_by_id(&self,  pg_connection: &mut PgConnection, container_meta_id: &i32) -> Result<models::container_meta::ContainerMeta, diesel::result::Error> {
        use models::schema::container_meta::dsl::*;

        let result = container_meta
        .filter(id.eq(container_meta_id))
        .first::<models::container_meta::ContainerMeta>(pg_connection)?;
        Ok(result)
    }

    fn update_file_meta_by_id(&self, pg_connection: &mut PgConnection, file_meta_id: &i32, in_file_meta: &models::file_meta::FileMeta) -> Result<models::file_meta::FileMeta, diesel::result::Error> {
        use models::schema::file_meta::dsl::*;

        let result = diesel::update(file_meta.filter(id.eq(file_meta_id)))
        .set((name.eq(&in_file_meta.name), file_type.eq(&in_file_meta.file_type), file_size_in_kb.eq(&in_file_meta.file_size_in_kb)))
        .returning(models::file_meta::FileMeta::as_returning())
        .get_result(pg_connection)?;

        Ok(result)
    }

    fn update_container_meta_by_id(&self, pg_connection: &mut PgConnection, container_meta_id: &i32, in_container_meta: &models::container_meta::ContainerMeta) -> Result<models::container_meta::ContainerMeta, diesel::result::Error> {
        use models::schema::container_meta::dsl::*;
        
        let result = diesel::update(container_meta.filter(id.eq(container_meta_id)))
        .set((date_time_created.eq(&in_container_meta.date_time_created), 
            date_time_updated.eq(&in_container_meta.date_time_updated),
            title.eq(&in_container_meta.title),
            description.eq(&in_container_meta.description),
            tags.eq(&in_container_meta.tags),
            file_meta_ids.eq(&in_container_meta.file_meta_ids)))
        .returning(models::container_meta::ContainerMeta::as_returning())
        .get_result(pg_connection)?;

        Ok(result)
    }

    async fn delete_models_by_container_id(&self, container_id: &i32) -> Result<(), diesel::result::Error> {
        
        Ok(())
    }
}



