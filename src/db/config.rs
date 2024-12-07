use sea_orm::DbErr;
use sea_orm_migration::{MigratorTrait, SchemaManager};

use crate::{migrator, DbPool};


pub async fn run(db: &DbPool) -> Result<(), DbErr> {
    // Drop and recreate the target database
    let schema_manager = SchemaManager::new(db.as_ref()); // To investigate the schema

    migrator::Migrator::refresh(db.as_ref()).await?;
    assert!(schema_manager.has_table("user").await?);
    // assert!(schema_manager.has_table("chef").await?);

    Ok(())
}