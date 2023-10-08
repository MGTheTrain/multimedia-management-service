use diesel::{ConnectionResult, PgConnection, Connection};

pub struct PsqlDataAccess {
    pg_connection: Option<ConnectionResult<PgConnection>>,
}

impl PsqlDataAccess {
    /// Method new for constructing an object from the struct PsqlDataAccess
    ///
    /// This method takes no parameters,
    /// and returns an PsqlDataAccess object
    fn new(database_url: &str) -> Self {
        PsqlDataAccess {
           pg_connection: Some(PgConnection::establish(&database_url)),
        }
    }

        
}

