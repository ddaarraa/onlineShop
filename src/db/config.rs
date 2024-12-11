// use sea_orm::DbErr;
use sea_orm_migration::{MigratorTrait, SchemaManager};
use sea_orm::{Database, DatabaseConnection};
use crate::{migrator, DbPool};
use std::{env, sync::Arc};


// pub async fn run(db: &DbPool) -> Result<(), DbErr> {
//     // Drop and recreate the target database
//     let schema_manager = SchemaManager::new(db.as_ref()); // To investigate the schema

//     migrator::Migrator::refresh(db.as_ref()).await?;
//     assert!(schema_manager.has_table("user").await?);
//     // assert!(schema_manager.has_table("chef").await?);

//     Ok(())
// }
pub async fn database_connection() -> Arc<DatabaseConnection> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Arc::new(Database::connect(&database_url).await.expect("Failed to connect to the database"))
}
