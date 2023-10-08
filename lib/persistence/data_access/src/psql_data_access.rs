use diesel::{ConnectionResult, PgConnection, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, result::Error, SelectableHelper};
use models::{file_meta::FileMeta, container_meta::ContainerMeta};
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

    async fn insert_file_meta(&self, in_file_meta: &models::file_meta::FileMeta) -> Result<models::file_meta::FileMeta, diesel::result::Error> {
        use models::schema::file_meta::dsl::*;
        
        let pg_connection = self.pg_connection.as_mut().ok_or(diesel::result::Error::NotFound)?;
        diesel::insert_into(file_meta::table)
        .values(&in_file_meta)
        .returning(models::file_meta::FileMeta::as_returning())
        .get_result(conn)
        .expect("Error saving file meta")
    }

    async fn insert_container_meta(&self, in_container_meta: &models::container_meta::ContainerMeta) -> Result<models::container_meta::ContainerMeta, diesel::result::Error> {
        use models::schema::container_meta::dsl::*;
        
        let pg_connection = self.pg_connection.as_mut().ok_or(diesel::result::Error::NotFound)?;
        diesel::insert_into(container_meta::table)
        .values(&in_container_meta)
        .returning(models::container_meta::ContainerMeta::as_returning())
        .get_result(conn)
        .expect("Error saving file meta")
    }

    fn get_file_meta_by_id(&self, file_meta_id: &i32) -> Result<models::file_meta::FileMeta, diesel::result::Error> {
        use models::schema::file_meta::dsl::*;
        
        let pg_connection = self.pg_connection.as_mut().ok_or(diesel::result::Error::NotFound)?;
        let result = file_meta
        .filter(id.eq(file_meta_id))
        .limit(1)
        .select(FileMeta::as_select())
        .load(pg_connection)
        .expect("Error loading file meta");
        Ok(result)
    }
    

    async fn get_container_meta_by_id(&self, container_meta_id: &i32) -> Result<models::container_meta::ContainerMeta, diesel::result::Error> {
        use models::schema::container_meta::dsl::*;

        let pg_connection = self.pg_connection.as_mut().ok_or(diesel::result::Error::NotFound)?;
        let result = container_meta
        .filter(id.eq(container_meta_id))
        .limit(1)
        .select(ContainerMeta::as_select())
        .load(pg_connection)
        .expect("Error loading container meta");
        Ok(result)
    }

    async fn update_file_meta_by_id(&self, file_meta_id: &i32, in_file_meta: &models::file_meta::FileMeta) -> Result<(), diesel::result::Error> {
        
        Ok(())
    }

    async fn update_container_meta_by_id(&self, container_meta_id: &i32, in_container_meta: &models::file_meta::FileMeta) -> Result<(), diesel::result::Error> {
        
        Ok(())
    }

    async fn delete_models_by_container_id(&self, container_id: &i32) -> Result<(), diesel::result::Error> {
        
        Ok(())
    }
}



