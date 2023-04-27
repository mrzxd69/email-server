use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct PoolDatabaseConnection {
    pub connection: DatabaseConnection
}


