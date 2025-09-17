use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::db::{error::Error, schema::Schema};

/// Database migration manager for the NFL prediction dashboard
pub struct MigrationManager;

impl MigrationManager {
    /// Run all migrations to set up the database
    pub async fn migrate(db: &Surreal<Client>) -> Result<(), Error> {
        println!("Starting database migration...");
        
        // Check if migration is needed
        if Self::needs_migration(db).await? {
            println!("Migration needed, initializing schema...");
            
            // Initialize schema
            Schema::initialize(db).await?;
            
            // Seed initial data
            Schema::seed_nfl_teams(db).await?;
            
            // Mark migration as complete
            Self::mark_migration_complete(db).await?;
            
            println!("Database migration completed successfully");
        } else {
            println!("Database is already up to date");
        }
        
        Ok(())
    }

    /// Check if migration is needed by looking for the teams table
    async fn needs_migration(db: &Surreal<Client>) -> Result<bool, Error> {
        // Try to query the teams table to see if it exists
        let result = db.query("SELECT * FROM teams LIMIT 1").await;
        
        match result {
            Ok(_) => {
                // Table exists, check if it has data
                let teams: Vec<serde_json::Value> = db.query("SELECT * FROM teams")
                    .await?
                    .take(0)?;
                
                // If we have fewer than 32 teams, we need to migrate
                Ok(teams.len() < 32)
            }
            Err(_) => {
                // Table doesn't exist, need migration
                Ok(true)
            }
        }
    }

    /// Mark migration as complete by creating a migration record
    async fn mark_migration_complete(db: &Surreal<Client>) -> Result<(), Error> {
        db.query(r#"
            CREATE migration_history SET {
                version: "1.0.0",
                description: "Initial schema and NFL teams",
                applied_at: time::now()
            }
        "#).await?;
        
        Ok(())
    }

    /// Reset the database by dropping all tables and re-migrating
    pub async fn reset(db: &Surreal<Client>) -> Result<(), Error> {
        println!("Resetting database...");
        
        // Drop all tables
        Schema::drop_all_tables(db).await?;
        
        // Run migration
        Self::migrate(db).await?;
        
        println!("Database reset completed");
        Ok(())
    }

    /// Get migration status
    pub async fn get_status(db: &Surreal<Client>) -> Result<MigrationStatus, Error> {
        todo!("Get the corresponding test_migration_manager test to pass!");
        // Try to count teams - this will fail if table doesn't exist or has no schema
        let count_result = db.query("SELECT count() FROM teams GROUP ALL").await;
        
        match count_result {
            Ok(mut response) => {
                let count_data: Vec<serde_json::Value> = response.take(0)?;
                
                if let Some(count_obj) = count_data.first() {
                    if let Some(count) = count_obj.get("count").and_then(|c| c.as_u64()) {
                        if count >= 32 {
                            Ok(MigrationStatus::Complete)
                        } else if count == 0 {
                            Ok(MigrationStatus::SchemaOnly)
                        } else {
                            Ok(MigrationStatus::Partial)
                        }
                    } else {
                        Ok(MigrationStatus::SchemaOnly)
                    }
                } else {
                    Ok(MigrationStatus::SchemaOnly)
                }
            }
            Err(_) => {
                // Count failed, likely table doesn't exist
                Ok(MigrationStatus::NotMigrated)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MigrationStatus {
    NotMigrated,
    SchemaOnly,
    Partial,
    Complete,
}

impl std::fmt::Display for MigrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationStatus::NotMigrated => write!(f, "Not migrated"),
            MigrationStatus::SchemaOnly => write!(f, "Schema created, no data"),
            MigrationStatus::Partial => write!(f, "Partially migrated"),
            MigrationStatus::Complete => write!(f, "Migration complete"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connect;

    #[tokio::test]
    async fn test_migration_manager() {
        let db = connect().await.expect("Failed to connect to database");
        
        // Clean slate
        let _ = Schema::drop_all_tables(&db).await;
        
        // Check initial status
        let status = MigrationManager::get_status(&db).await.expect("Failed to get status");
        assert_eq!(status, MigrationStatus::NotMigrated);
        
        // Run migration
        MigrationManager::migrate(&db).await.expect("Migration failed");
        
        // Check final status
        let status = MigrationManager::get_status(&db).await.expect("Failed to get status");
        assert_eq!(status, MigrationStatus::Complete);
        
        // Running migration again should be idempotent
        MigrationManager::migrate(&db).await.expect("Second migration failed");
        
        // Clean up
        let _ = Schema::drop_all_tables(&db).await;
    }

    #[tokio::test]
    async fn test_migration_reset() {
        let db = connect().await.expect("Failed to connect to database");
        
        // Set up initial state
        MigrationManager::migrate(&db).await.expect("Initial migration failed");
        
        // Reset
        MigrationManager::reset(&db).await.expect("Reset failed");
        
        // Verify reset worked
        let status = MigrationManager::get_status(&db).await.expect("Failed to get status");
        assert_eq!(status, MigrationStatus::Complete);
        
        // Clean up
        let _ = Schema::drop_all_tables(&db).await;
    }

    #[tokio::test]
    async fn test_needs_migration() {
        let db = connect().await.expect("Failed to connect to database");
        
        // Clean slate
        let _ = Schema::drop_all_tables(&db).await;
        
        // Should need migration
        let needs = MigrationManager::needs_migration(&db).await.expect("Failed to check migration need");
        assert!(needs);
        
        // Create schema but no data
        Schema::initialize(&db).await.expect("Failed to initialize schema");
        let needs = MigrationManager::needs_migration(&db).await.expect("Failed to check migration need");
        assert!(needs); // Still needs migration for data
        
        // Add data
        Schema::seed_nfl_teams(&db).await.expect("Failed to seed teams");
        let needs = MigrationManager::needs_migration(&db).await.expect("Failed to check migration need");
        assert!(!needs); // No longer needs migration
        
        // Clean up
        let _ = Schema::drop_all_tables(&db).await;
    }
}