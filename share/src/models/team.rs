use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::game::{GameResult, GameOutcome};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub abbreviation: String,
    pub conference: Option<String>,
    pub division: Option<String>,
    pub stats: TeamStats,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeamStats {
    pub offensive_rating: f64,
    pub defensive_rating: f64,
    pub points_per_game: f64,
    pub points_allowed_per_game: f64,
    pub yards_per_game: f64,
    pub yards_allowed_per_game: f64,
    pub turnover_differential: i32,
    pub recent_form: Vec<GameResult>,
    pub injury_report: Vec<PlayerInjury>,
    pub season: u16,
    pub games_played: u8,
    pub wins: u8,
    pub losses: u8,
    pub ties: u8,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerInjury {
    pub player_id: String,
    pub player_name: String,
    pub position: String,
    pub injury_type: String,
    pub status: InjuryStatus,
    pub estimated_return: Option<DateTime<Utc>>,
    pub impact_rating: f64, // 0.0 to 1.0, where 1.0 is highest impact
    pub reported_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InjuryStatus {
    Questionable,
    Doubtful,
    Out,
    InjuredReserve,
    Healthy,
}

impl Team {
    pub fn new(name: String, abbreviation: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            abbreviation,
            conference: None,
            division: None,
            stats: TeamStats::default(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_conference_division(
        name: String,
        abbreviation: String,
        conference: String,
        division: String,
    ) -> Self {
        let mut team = Self::new(name, abbreviation);
        team.conference = Some(conference);
        team.division = Some(division);
        team
    }

    pub fn update_stats(&mut self, stats: TeamStats) {
        self.stats = stats;
        self.updated_at = Utc::now();
    }

    pub fn add_injury(&mut self, injury: PlayerInjury) {
        self.stats.injury_report.push(injury);
        self.updated_at = Utc::now();
    }

    pub fn get_win_percentage(&self) -> f64 {
        let total_games = self.stats.games_played;
        if total_games == 0 {
            return 0.0;
        }
        self.stats.wins as f64 / total_games as f64
    }

    pub fn get_recent_form_wins(&self, last_n_games: usize) -> usize {
        self.stats
            .recent_form
            .iter()
            .rev()
            .take(last_n_games)
            .filter(|result| result.result == GameOutcome::Win)
            .count()
    }

    pub fn get_active_injuries(&self) -> Vec<&PlayerInjury> {
        self.stats
            .injury_report
            .iter()
            .filter(|injury| !matches!(injury.status, InjuryStatus::Healthy))
            .collect()
    }
}

impl Default for TeamStats {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            offensive_rating: 0.0,
            defensive_rating: 0.0,
            points_per_game: 0.0,
            points_allowed_per_game: 0.0,
            yards_per_game: 0.0,
            yards_allowed_per_game: 0.0,
            turnover_differential: 0,
            recent_form: Vec::new(),
            injury_report: Vec::new(),
            season: 2024,
            games_played: 0,
            wins: 0,
            losses: 0,
            ties: 0,
            last_updated: now,
        }
    }
}

impl TeamStats {
    pub fn new(season: u16) -> Self {
        Self {
            season,
            ..Default::default()
        }
    }

    pub fn update_record(&mut self, outcome: GameOutcome) {
        match outcome {
            GameOutcome::Win => self.wins += 1,
            GameOutcome::Loss => self.losses += 1,
            GameOutcome::Tie => self.ties += 1,
        }
        self.games_played += 1;
        self.last_updated = Utc::now();
    }

    pub fn calculate_strength_of_schedule(&self) -> f64 {
        if self.recent_form.is_empty() {
            return 0.5; // Neutral if no data
        }

        // Simple calculation based on opponent performance
        let total_opponent_wins: f64 = self.recent_form
            .iter()
            .map(|game| {
                // This would need opponent win percentage in real implementation
                // For now, return a placeholder calculation
                if game.result == GameOutcome::Loss {
                    0.6 // Assume teams we lost to are stronger
                } else {
                    0.4 // Assume teams we beat are weaker
                }
            })
            .sum();

        total_opponent_wins / self.recent_form.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_creation() {
        let team = Team::new("Kansas City Chiefs".to_string(), "KC".to_string());
        
        assert_eq!(team.name, "Kansas City Chiefs");
        assert_eq!(team.abbreviation, "KC");
        assert!(team.conference.is_none());
        assert!(team.division.is_none());
        assert!(!team.id.is_empty());
        assert_eq!(team.stats.games_played, 0);
    }

    #[test]
    fn test_team_with_conference_division() {
        let team = Team::with_conference_division(
            "Kansas City Chiefs".to_string(),
            "KC".to_string(),
            "AFC".to_string(),
            "West".to_string(),
        );
        
        assert_eq!(team.conference, Some("AFC".to_string()));
        assert_eq!(team.division, Some("West".to_string()));
    }

    #[test]
    fn test_team_stats_default() {
        let stats = TeamStats::default();
        
        assert_eq!(stats.offensive_rating, 0.0);
        assert_eq!(stats.defensive_rating, 0.0);
        assert_eq!(stats.games_played, 0);
        assert_eq!(stats.wins, 0);
        assert_eq!(stats.losses, 0);
        assert_eq!(stats.ties, 0);
        assert_eq!(stats.season, 2024);
    }

    #[test]
    fn test_team_stats_update_record() {
        let mut stats = TeamStats::new(2024);
        
        stats.update_record(GameOutcome::Win);
        assert_eq!(stats.wins, 1);
        assert_eq!(stats.games_played, 1);
        
        stats.update_record(GameOutcome::Loss);
        assert_eq!(stats.losses, 1);
        assert_eq!(stats.games_played, 2);
        
        stats.update_record(GameOutcome::Tie);
        assert_eq!(stats.ties, 1);
        assert_eq!(stats.games_played, 3);
    }

    #[test]
    fn test_win_percentage() {
        let mut team = Team::new("Test Team".to_string(), "TT".to_string());
        
        // No games played
        assert_eq!(team.get_win_percentage(), 0.0);
        
        // Update stats with some wins and losses
        team.stats.wins = 3;
        team.stats.losses = 1;
        team.stats.games_played = 4;
        
        assert_eq!(team.get_win_percentage(), 0.75);
    }

    #[test]
    fn test_injury_management() {
        let mut team = Team::new("Test Team".to_string(), "TT".to_string());
        
        let injury = PlayerInjury {
            player_id: "player-1".to_string(),
            player_name: "John Doe".to_string(),
            position: "QB".to_string(),
            injury_type: "Ankle".to_string(),
            status: InjuryStatus::Questionable,
            estimated_return: None,
            impact_rating: 0.8,
            reported_at: Utc::now(),
        };
        
        team.add_injury(injury.clone());
        
        assert_eq!(team.stats.injury_report.len(), 1);
        assert_eq!(team.get_active_injuries().len(), 1);
        
        // Add a healthy player (should not count as active injury)
        let healthy_injury = PlayerInjury {
            status: InjuryStatus::Healthy,
            ..injury
        };
        
        team.add_injury(healthy_injury);
        assert_eq!(team.stats.injury_report.len(), 2);
        assert_eq!(team.get_active_injuries().len(), 1); // Still only 1 active
    }

    #[test]
    fn test_recent_form_wins() {
        let mut team = Team::new("Test Team".to_string(), "TT".to_string());
        
        // Add some recent game results
        let game_results = vec![
            GameResult {
                game_id: "1".to_string(),
                team_id: team.id.clone(),
                opponent_id: "opp1".to_string(),
                points_scored: 24,
                points_allowed: 21,
                is_home: true,
                result: GameOutcome::Win,
                game_date: Utc::now(),
            },
            GameResult {
                game_id: "2".to_string(),
                team_id: team.id.clone(),
                opponent_id: "opp2".to_string(),
                points_scored: 14,
                points_allowed: 28,
                is_home: false,
                result: GameOutcome::Loss,
                game_date: Utc::now(),
            },
            GameResult {
                game_id: "3".to_string(),
                team_id: team.id.clone(),
                opponent_id: "opp3".to_string(),
                points_scored: 31,
                points_allowed: 17,
                is_home: true,
                result: GameOutcome::Win,
                game_date: Utc::now(),
            },
        ];
        
        team.stats.recent_form = game_results;
        
        assert_eq!(team.get_recent_form_wins(3), 2); // 2 wins in last 3 games
        assert_eq!(team.get_recent_form_wins(2), 1); // 1 win in last 2 games
    }

    #[test]
    fn test_team_serialization() {
        let team = Team::new("Kansas City Chiefs".to_string(), "KC".to_string());
        
        let serialized = serde_json::to_string(&team).expect("Failed to serialize team");
        let deserialized: Team = serde_json::from_str(&serialized).expect("Failed to deserialize team");
        
        assert_eq!(team, deserialized);
    }

    #[test]
    fn test_injury_status_variants() {
        let statuses = vec![
            InjuryStatus::Questionable,
            InjuryStatus::Doubtful,
            InjuryStatus::Out,
            InjuryStatus::InjuredReserve,
            InjuryStatus::Healthy,
        ];
        
        for status in statuses {
            let serialized = serde_json::to_string(&status).expect("Failed to serialize status");
            let deserialized: InjuryStatus = serde_json::from_str(&serialized).expect("Failed to deserialize status");
            assert_eq!(status, deserialized);
        }
    }
}