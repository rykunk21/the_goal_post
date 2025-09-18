use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::team::Team;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Game {
    pub id: String,
    pub home_team: Team,
    pub away_team: Team,
    pub game_time: DateTime<Utc>,
    pub week: u8,
    pub season: u16,
    pub status: GameStatus,
    pub home_score: Option<u8>,
    pub away_score: Option<u8>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameStatus {
    Scheduled,
    InProgress,
    Completed,
    Postponed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameResult {
    pub game_id: String,
    pub team_id: String,
    pub opponent_id: String,
    pub points_scored: u8,
    pub points_allowed: u8,
    pub is_home: bool,
    pub result: GameOutcome,
    pub game_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameOutcome {
    Win,
    Loss,
    Tie,
}

impl Game {
    pub fn new(
        home_team: Team,
        away_team: Team,
        game_time: DateTime<Utc>,
        week: u8,
        season: u16,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            home_team,
            away_team,
            game_time,
            week,
            season,
            status: GameStatus::Scheduled,
            home_score: None,
            away_score: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, GameStatus::Completed)
    }

    pub fn is_upcoming(&self) -> bool {
        matches!(self.status, GameStatus::Scheduled) && self.game_time > Utc::now()
    }

    pub fn update_score(&mut self, home_score: u8, away_score: u8) {
        self.home_score = Some(home_score);
        self.away_score = Some(away_score);
        self.updated_at = Utc::now();
    }

    pub fn set_status(&mut self, status: GameStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::team::{Team, TeamStats};

    fn create_test_team(name: &str, abbreviation: &str) -> Team {
        Team::new(name.to_string(), abbreviation.to_string())
    }

    #[test]
    fn test_game_creation() {
        let home_team = create_test_team("Kansas City Chiefs", "KC");
        let away_team = create_test_team("Buffalo Bills", "BUF");
        let game_time = Utc::now();
        
        let game = Game::new(home_team.clone(), away_team.clone(), game_time, 1, 2024);
        
        assert_eq!(game.home_team, home_team);
        assert_eq!(game.away_team, away_team);
        assert_eq!(game.game_time, game_time);
        assert_eq!(game.week, 1);
        assert_eq!(game.season, 2024);
        assert_eq!(game.status, GameStatus::Scheduled);
        assert!(game.home_score.is_none());
        assert!(game.away_score.is_none());
        assert!(!game.id.is_empty());
    }

    #[test]
    fn test_game_status_checks() {
        let home_team = create_test_team("Kansas City Chiefs", "KC");
        let away_team = create_test_team("Buffalo Bills", "BUF");
        let future_time = Utc::now() + chrono::Duration::hours(1);
        
        let mut game = Game::new(home_team, away_team, future_time, 1, 2024);
        
        assert!(game.is_upcoming());
        assert!(!game.is_completed());
        
        game.set_status(GameStatus::Completed);
        assert!(game.is_completed());
        assert!(!game.is_upcoming());
    }

    #[test]
    fn test_game_score_update() {
        let home_team = create_test_team("Kansas City Chiefs", "KC");
        let away_team = create_test_team("Buffalo Bills", "BUF");
        let game_time = Utc::now();
        
        let mut game = Game::new(home_team, away_team, game_time, 1, 2024);
        let initial_updated_at = game.updated_at;
        
        // Small delay to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        game.update_score(24, 21);
        
        assert_eq!(game.home_score, Some(24));
        assert_eq!(game.away_score, Some(21));
        assert!(game.updated_at > initial_updated_at);
    }

    #[test]
    fn test_game_serialization() {
        let home_team = create_test_team("Kansas City Chiefs", "KC");
        let away_team = create_test_team("Buffalo Bills", "BUF");
        let game_time = Utc::now();
        
        let game = Game::new(home_team, away_team, game_time, 1, 2024);
        
        let serialized = serde_json::to_string(&game).expect("Failed to serialize game");
        let deserialized: Game = serde_json::from_str(&serialized).expect("Failed to deserialize game");
        
        assert_eq!(game, deserialized);
    }

    #[test]
    fn test_game_result_creation() {
        let game_result = GameResult {
            game_id: "test-game-id".to_string(),
            team_id: "team-1".to_string(),
            opponent_id: "team-2".to_string(),
            points_scored: 24,
            points_allowed: 21,
            is_home: true,
            result: GameOutcome::Win,
            game_date: Utc::now(),
        };
        
        assert_eq!(game_result.result, GameOutcome::Win);
        assert!(game_result.is_home);
        assert_eq!(game_result.points_scored, 24);
        assert_eq!(game_result.points_allowed, 21);
    }
}