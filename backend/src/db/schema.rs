// Schema definitions removed - using schemaless storage
// This file is kept for reference but no longer used for formal schema management

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::db::error::Error;
use share::models::Team;

/// Simple data seeding utilities for schemaless storage
pub struct DataSeeder;

impl DataSeeder {
    /// Seed some basic NFL teams for testing (optional - not required for schemaless storage)
    pub async fn seed_sample_teams(db: &Surreal<Client>) -> Result<(), Error> {
        println!("Seeding sample NFL teams...");
        
        let sample_teams = vec![
            Team::with_conference_division(
                "Buffalo Bills".to_string(),
                "BUF".to_string(),
                "AFC".to_string(),
                "East".to_string(),
            ),
            Team::with_conference_division(
                "Miami Dolphins".to_string(),
                "MIA".to_string(),
                "AFC".to_string(),
                "East".to_string(),
            ),
            Team::with_conference_division(
                "New England Patriots".to_string(),
                "NE".to_string(),
                "AFC".to_string(),
                "East".to_string(),
            ),
            Team::with_conference_division(
                "New York Jets".to_string(),
                "NYJ".to_string(),
                "AFC".to_string(),
                "East".to_string(),
            ),
            Team::with_conference_division(
                "Atlanta Falcons".to_string(),
                "ATL".to_string(),
                "NFC".to_string(),
                "South".to_string(),
            ),
            Team::with_conference_division(
                "Carolina Panthers".to_string(),
                "CAR".to_string(),
                "NFC".to_string(),
                "South".to_string(),
            ),
        ];

        for team in sample_teams {
            let team_name = team.name.clone();
            let _: Option<serde_json::Value> = db.create("teams").content(team).await?;
            println!("Seeded team: {}", team_name);
        }
        
        println!("Sample NFL teams seeded successfully");
        Ok(())
    }

    /// Check if we have any teams in the database
    pub async fn has_teams(db: &Surreal<Client>) -> Result<bool, Error> {
        let teams: Vec<Team> = db.select("teams").await?;
        Ok(!teams.is_empty())
    }

    /// Get count of teams in database
    pub async fn team_count(db: &Surreal<Client>) -> Result<usize, Error> {
        let teams: Vec<Team> = db.select("teams").await?;
        Ok(teams.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DatabaseManager;

    #[tokio::test]
    async fn test_seed_sample_teams() {
        let db_manager = DatabaseManager::new().await.expect("Failed to connect");
        
        // Clear existing teams first
        let _result = db_manager.db.query("DELETE FROM teams").await;
        
        // Seed sample teams
        let result = DataSeeder::seed_sample_teams(&db_manager.db).await;
        assert!(result.is_ok());
        
        // Check that teams were seeded
        let has_teams = DataSeeder::has_teams(&db_manager.db).await.expect("Failed to check teams");
        assert!(has_teams);
        
        let count = DataSeeder::team_count(&db_manager.db).await.expect("Failed to count teams");
        assert!(count > 0);
        
        // Clean up
        let _result = db_manager.db.query("DELETE FROM teams").await;
    }

    #[tokio::test]
    async fn test_team_count() {
        let db_manager = DatabaseManager::new().await.expect("Failed to connect");
        
        // Clear existing teams first
        let _result = db_manager.db.query("DELETE FROM teams").await;
        
        // Should have no teams initially
        let count = DataSeeder::team_count(&db_manager.db).await.expect("Failed to count teams");
        assert_eq!(count, 0);
        
        // Seed teams
        DataSeeder::seed_sample_teams(&db_manager.db).await.expect("Failed to seed teams");
        
        // Should have teams now
        let count = DataSeeder::team_count(&db_manager.db).await.expect("Failed to count teams");
        assert!(count > 0);
        
        // Clean up
        let _result = db_manager.db.query("DELETE FROM teams").await;
    }
}