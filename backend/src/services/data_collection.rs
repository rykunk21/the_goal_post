use crate::db::{DatabaseManager, error::Error};
use share::models::{Team, Game, BettingLine, GamePrediction};

/// Simple data collection service using schemaless storage
pub struct DataCollectionService {
    db: DatabaseManager,
}

impl DataCollectionService {
    pub async fn new() -> Result<Self, surrealdb::Error> {
        let db = DatabaseManager::new().await?;
        Ok(Self { db })
    }

    /// Store a team with validation
    pub async fn store_team(&self, team: Team) -> Result<String, Error> {
        let validated_team = team.validate_and_create()
            .map_err(|_| Error::EntryExists)?; // Reusing existing error for validation
        
        let record_id = self.db.store("teams", validated_team).await?;
        Ok(record_id.to_string())
    }

    /// Get all teams
    pub async fn get_all_teams(&self) -> Result<Vec<Team>, Error> {
        let teams = self.db.get_all("teams").await?;
        Ok(teams)
    }

    /// Store a game
    pub async fn store_game(&self, game: Game) -> Result<String, Error> {
        let record_id = self.db.store("games", game).await?;
        Ok(record_id.to_string())
    }

    /// Get all games
    pub async fn get_all_games(&self) -> Result<Vec<Game>, Error> {
        let games = self.db.get_all("games").await?;
        Ok(games)
    }

    /// Store a betting line
    pub async fn store_betting_line(&self, line: BettingLine) -> Result<String, Error> {
        let record_id = self.db.store("betting_lines", line).await?;
        Ok(record_id.to_string())
    }

    /// Store a prediction
    pub async fn store_prediction(&self, prediction: GamePrediction) -> Result<String, Error> {
        let record_id = self.db.store("predictions", prediction).await?;
        Ok(record_id.to_string())
    }

    /// Health check for the service
    pub async fn health_check(&self) -> Result<bool, Error> {
        let health = self.db.health_check().await?;
        Ok(health)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_data_collection_service() {
        let service = DataCollectionService::new().await.expect("Failed to create service");
        
        // Test health check
        assert!(service.health_check().await.unwrap());
        
        // Test storing a team
        let team = Team::new("Test Service Team".to_string(), "TST".to_string());
        let team_id = service.store_team(team).await.expect("Failed to store team");
        assert!(!team_id.is_empty());
        
        // Test getting all teams
        let teams = service.get_all_teams().await.expect("Failed to get teams");
        assert!(teams.len() >= 1);
        
        // Clean up - delete the test team
        let _: Option<Team> = service.db.delete("teams", &team_id).await.expect("Failed to delete test team");
    }
}