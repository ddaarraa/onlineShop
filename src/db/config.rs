
use sea_orm::DbErr;
use sea_orm_migration::{MigratorTrait, SchemaManager};
use sea_orm::{Database, DatabaseConnection};
use crate::migrator;
use crate::config;
use std::sync::Arc;
use std::error::Error;


pub async fn run(db: &DbPool) -> Result<(), DbErr> {
    // Drop and recreate the target database
    let schema_manager = SchemaManager::new(db.as_ref()); // To investigate the schema

    migrator::Migrator::up(db.as_ref(), None ).await?;
    assert!(schema_manager.has_table("user").await?);
    // assert!(schema_manager.has_table("chef").await?);

    Ok(())
}
pub async fn database_connection() -> Result<Arc<DatabaseConnection>, Box<dyn Error>> {

    let database_url = config::env_config::get_database_url_from_config();
   
    let database_url = match database_url {
        Ok(database_url) => { database_url },
        Err(err) => {err.to_string()},
    };
    // print!("database url : {}",database_url);
    let connection = Database::connect(&database_url).await?;
    Ok(Arc::new(connection))

}

type DbPool = Arc<DatabaseConnection>;


