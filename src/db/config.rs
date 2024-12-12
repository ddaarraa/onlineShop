// use sea_orm::DbErr;
// use sea_orm_migration::{MigratorTrait, SchemaManager};
use sea_orm::{Database, DatabaseConnection};
// use crate::{migrator, DbPool};
use crate::config;
use std::sync::Arc;
use std::error::Error;


// pub async fn run(db: &DbPool) -> Result<(), DbErr> {
//     // Drop and recreate the target database
//     let schema_manager = SchemaManager::new(db.as_ref()); // To investigate the schema

//     migrator::Migrator::refresh(db.as_ref()).await?;
//     assert!(schema_manager.has_table("user").await?);
//     // assert!(schema_manager.has_table("chef").await?);

//     Ok(())
// }
pub async fn database_connection() -> Result<Arc<DatabaseConnection>, Box<dyn Error>> {

    let env_config = config::env_config::get_env_config();
    let config_guard = env_config.lock().unwrap();
   
    let database_url = config_guard.database_url.clone()
        .ok_or("DATABASE_URL is not set in the environment")?;

    // print!("database url : {}",database_url);
    let connection = Database::connect(&database_url).await?;
    Ok(Arc::new(connection))

}

