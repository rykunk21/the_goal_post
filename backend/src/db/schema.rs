use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::db::error::Error;

/// Database schema definitions and migration scripts for the NFL prediction dashboard
pub struct Schema;

impl Schema {
    /// Initialize the database schema with all required tables and indexes
    pub async fn initialize(db: &Surreal<Client>) -> Result<(), Error> {
        // Create tables
        Self::create_teams_table(db).await?;
        Self::create_games_table(db).await?;
        Self::create_team_stats_table(db).await?;
        Self::create_player_injuries_table(db).await?;
        Self::create_game_results_table(db).await?;
        Self::create_betting_lines_table(db).await?;
        Self::create_betting_providers_table(db).await?;
        Self::create_game_predictions_table(db).await?;
        Self::create_line_comparisons_table(db).await?;
        Self::create_value_opportunities_table(db).await?;

        // Create indexes
        Self::create_indexes(db).await?;

        println!("Database schema initialized successfully");
        Ok(())
    }

    /// Create the teams table
    async fn create_teams_table(db: &Surreal<Client>) -> Result<(), Error> {
        db.query(r#"
            DEFINE TABLE teams SCHEMAFULL;
            DEFINE FIELD id ON TABLE teams TYPE string;
            DEFINE FIELD name ON TABLE teams TYPE string;
            DEFINE FIELD abbreviation ON TABLE teams TYPE string;
            DEFINE FIELD conference ON TABLE teams TYPE option<string>;
            DEFINE FIELD division ON TABLE teams TYPE option<string>;
            DEFINE FIELD created_at ON TABLE teams TYPE datetime;
            DEFINE FIELD updated_at ON TABLE teams TYPE datetime;
        "#).await?;
        Ok(())
    }

    /// Create the games table
    async fn create_games_table(db: &Surreal<Client>) -> Result<(), Error> {
        db.query(r#"
            DEFINE TABLE games SCHEMAFULL;
            DEFINE FIELD id ON TABLE games TYPE string;
            DEFINE FIELD home_team_id ON TABLE games TYPE string;
            DEFINE FIELD away_team_id ON TABLE games TYPE string;
            DEFINE FIELD game_time ON TABLE games TYPE datetime;
            DEFINE FIELD week ON TABLE games TYPE int;
            DEFINE FIELD season ON TABLE games TYPE int;
            DEFINE FIELD status ON TABLE games TYPE string;
            DEFINE FIELD home_score ON TABLE games TYPE option<int>;
            DEFINE FIELD away_score ON TABLE games TYPE option<int>;
            DEFINE FIELD created_at ON TABLE games TYPE datetime;
            DEFINE FIELD updated_at ON TABLE games TYPE datetime;
        "#).await?;
        Ok(())
    }

    /// Create the team_stats table
    async fn create_team_stats_table(db: &Surreal<Client>) -> Result<(), Error> {
        db.query(r#"
            DEFINE TABLE team_stats SCHEMAFULL;
            DEFINE FIELD team_id ON TABLE team_stats TYPE string;
            DEFINE FIELD season ON TABLE team_stats TYPE int;
            DEFINE FIELD offensive_rating ON TABLE team_stats TYPE float;
            DEFINE FIELD defensive_rating ON TABLE team_stats TYPE float;
            DEFINE FIELD points_per_game ON TABLE team_stats TYPE float;
            DEFINE FIELD points_allowed_per_game ON TABLE team_stats TYPE float;
            DEFINE FIELD yards_per_game ON TABLE team_stats TYPE float;
            DEFINE FIELD yards_allowed_per_game ON TABLE team_stats TYPE float;
            DEFINE FIELD turnover_differential ON TABLE team_stats TYPE int;
            DEFINE FIELD games_played ON TABLE team_stats TYPE int;
            DEFINE FIELD wins ON TABLE team_stats TYPE int;
            DEFINE FIELD losses ON TABLE team_stats TYPE int;
            DEFINE FIELD ties ON TABLE team_stats TYPE int;
            DEFINE FIELD last_updated ON TABLE team_stats TYPE datetime;
        "#).await?;
        Ok(())
    }

    /// Create the player_injuries table
    async fn create_player_injuries_table(db: &Surreal<Client>) -> Result<(), Error> {
        db.query(r#"
            DEFINE TABLE player_injuries SCHEMAFULL;
            DEFINE FIELD player_id ON TABLE player_injuries TYPE string;
            DEFINE FIELD team_id ON TABLE player_injuries TYPE string;
            DEFINE FIELD player_name ON TABLE player_injuries TYPE string;
            DEFINE FIELD position ON TABLE player_injuries TYPE string;
            DEFINE FIELD injury_type ON TABLE player_injuries TYPE string;
            DEFINE FIELD status ON TABLE player_injuries TYPE string;
            DEFINE FIELD estimated_return ON TABLE player_injuries TYPE option<datetime>;
            DEFINE FIELD impact_rating ON TABLE player_injuries TYPE float;
            DEFINE FIELD reported_at ON TABLE player_injuries TYPE datetime;
        "#).await?;
        Ok(())
    }

    /// Create the game_results table
    async fn create_game_results_table(db: &Surreal<Client>) -> Result<(), Error> {
        db.query(r#"
            DEFINE TABLE game_results SCHEMAFULL;
            DEFINE FIELD game_id ON TABLE game_results TYPE string;
            DEFINE FIELD team_id ON TABLE game_results TYPE string;
            DEFINE FIELD opponent_id ON TABLE game_results TYPE string;
            DEFINE FIELD points_scored ON TABLE game_results TYPE int;
            DEFINE FIELD points_allowed ON TABLE game_results TYPE int;
            DEFINE FIELD is_home ON TABLE game_results TYPE bool;
            DEFINE FIELD result ON TABLE game_results TYPE string;
            DEFINE FIELD game_date ON TABLE game_results TYPE datetime;
        "#).await?;
        Ok(())
    }

    /// Create the betting_lines table
    async fn create_betting_lines_table(db: &Surreal<Client>) -> Result<(), Error> {
        db.query(r#"
            DEFINE TABLE betting_lines SCHEMAFULL;
            DEFINE FIELD id ON TABLE betting_lines TYPE string;
            DEFINE FIELD game_id ON TABLE betting_lines TYPE string;
            DEFINE FIELD provider ON TABLE betting_lines TYPE string;
            DEFINE FIELD spread ON TABLE betting_lines TYPE float;
            DEFINE FIELD total ON TABLE betting_lines TYPE float;
            DEFINE FIELD moneyline_home ON TABLE betting_lines TYPE int;
            DEFINE FIELD moneyline_away ON TABLE betting_lines TYPE int;
            DEFINE FIELD timestamp ON TABLE betting_lines TYPE datetime;
            DEFINE FIELD is_active ON TABLE betting_lines TYPE bool;
        "#).await?;
        Ok(())
    }

    /// Create the betting_providers table
    async fn create_betting_providers_table(db: &Surreal<Client>) -> Result<(), Error> {
        db.query(r#"
            DEFINE TABLE betting_providers SCHEMAFULL;
            DEFINE FIELD id ON TABLE betting_providers TYPE string;
            DEFINE FIELD name ON TABLE betting_providers TYPE string;
            DEFINE FIELD api_endpoint ON TABLE betting_providers TYPE string;
            DEFINE FIELD is_active ON TABLE betting_providers TYPE bool;
            DEFINE FIELD rate_limit_per_minute ON TABLE betting_providers TYPE int;
            DEFINE FIELD last_request_at ON TABLE betting_providers TYPE option<datetime>;
            DEFINE FIELD created_at ON TABLE betting_providers TYPE datetime;
        "#).await?;
        Ok(())
    }

    /// Create the game_predictions table
    async fn create_game_predictions_table(db: &Surreal<Client>) -> Result<(), Error> {
        db.query(r#"
            DEFINE TABLE game_predictions SCHEMAFULL;
            DEFINE FIELD id ON TABLE game_predictions TYPE string;
            DEFINE FIELD game_id ON TABLE game_predictions TYPE string;
            DEFINE FIELD home_score_mean ON TABLE game_predictions TYPE float;
            DEFINE FIELD home_score_std_dev ON TABLE game_predictions TYPE float;
            DEFINE FIELD away_score_mean ON TABLE game_predictions TYPE float;
            DEFINE FIELD away_score_std_dev ON TABLE game_predictions TYPE float;
            DEFINE FIELD spread_prediction ON TABLE game_predictions TYPE float;
            DEFINE FIELD total_prediction ON TABLE game_predictions TYPE float;
            DEFINE FIELD confidence_lower ON TABLE game_predictions TYPE float;
            DEFINE FIELD confidence_upper ON TABLE game_predictions TYPE float;
            DEFINE FIELD confidence_level ON TABLE game_predictions TYPE float;
            DEFINE FIELD generated_at ON TABLE game_predictions TYPE datetime;
        "#).await?;
        Ok(())
    }

    /// Create the line_comparisons table
    async fn create_line_comparisons_table(db: &Surreal<Client>) -> Result<(), Error> {
        db.query(r#"
            DEFINE TABLE line_comparisons SCHEMAFULL;
            DEFINE FIELD id ON TABLE line_comparisons TYPE string;
            DEFINE FIELD game_id ON TABLE line_comparisons TYPE string;
            DEFINE FIELD betting_line_id ON TABLE line_comparisons TYPE string;
            DEFINE FIELD prediction_id ON TABLE line_comparisons TYPE string;
            DEFINE FIELD spread_difference ON TABLE line_comparisons TYPE float;
            DEFINE FIELD total_difference ON TABLE line_comparisons TYPE float;
            DEFINE FIELD value_score ON TABLE line_comparisons TYPE float;
            DEFINE FIELD created_at ON TABLE line_comparisons TYPE datetime;
        "#).await?;
        Ok(())
    }

    /// Create the value_opportunities table
    async fn create_value_opportunities_table(db: &Surreal<Client>) -> Result<(), Error> {
        db.query(r#"
            DEFINE TABLE value_opportunities SCHEMAFULL;
            DEFINE FIELD id ON TABLE value_opportunities TYPE string;
            DEFINE FIELD game_id ON TABLE value_opportunities TYPE string;
            DEFINE FIELD opportunity_type ON TABLE value_opportunities TYPE string;
            DEFINE FIELD confidence ON TABLE value_opportunities TYPE float;
            DEFINE FIELD expected_value ON TABLE value_opportunities TYPE float;
            DEFINE FIELD recommendation ON TABLE value_opportunities TYPE string;
            DEFINE FIELD betting_line_id ON TABLE value_opportunities TYPE string;
            DEFINE FIELD created_at ON TABLE value_opportunities TYPE datetime;
            DEFINE FIELD expires_at ON TABLE value_opportunities TYPE option<datetime>;
        "#).await?;
        Ok(())
    }

    /// Create database indexes for performance
    async fn create_indexes(db: &Surreal<Client>) -> Result<(), Error> {
        db.query(r#"
            -- Team indexes
            DEFINE INDEX team_abbreviation ON TABLE teams COLUMNS abbreviation UNIQUE;
            DEFINE INDEX team_name ON TABLE teams COLUMNS name;
            
            -- Game indexes
            DEFINE INDEX game_season_week ON TABLE games COLUMNS season, week;
            DEFINE INDEX game_time ON TABLE games COLUMNS game_time;
            DEFINE INDEX game_teams ON TABLE games COLUMNS home_team_id, away_team_id;
            DEFINE INDEX game_status ON TABLE games COLUMNS status;
            
            -- Team stats indexes
            DEFINE INDEX team_stats_season ON TABLE team_stats COLUMNS team_id, season UNIQUE;
            DEFINE INDEX team_stats_updated ON TABLE team_stats COLUMNS last_updated;
            
            -- Player injury indexes
            DEFINE INDEX injury_team ON TABLE player_injuries COLUMNS team_id;
            DEFINE INDEX injury_status ON TABLE player_injuries COLUMNS status;
            DEFINE INDEX injury_reported ON TABLE player_injuries COLUMNS reported_at;
            
            -- Game results indexes
            DEFINE INDEX result_game ON TABLE game_results COLUMNS game_id;
            DEFINE INDEX result_team ON TABLE game_results COLUMNS team_id;
            DEFINE INDEX result_date ON TABLE game_results COLUMNS game_date;
            
            -- Betting line indexes
            DEFINE INDEX line_game ON TABLE betting_lines COLUMNS game_id;
            DEFINE INDEX line_provider ON TABLE betting_lines COLUMNS provider;
            DEFINE INDEX line_timestamp ON TABLE betting_lines COLUMNS timestamp;
            DEFINE INDEX line_active ON TABLE betting_lines COLUMNS is_active;
            
            -- Prediction indexes
            DEFINE INDEX prediction_game ON TABLE game_predictions COLUMNS game_id;
            DEFINE INDEX prediction_generated ON TABLE game_predictions COLUMNS generated_at;
            
            -- Comparison indexes
            DEFINE INDEX comparison_game ON TABLE line_comparisons COLUMNS game_id;
            DEFINE INDEX comparison_created ON TABLE line_comparisons COLUMNS created_at;
            
            -- Value opportunity indexes
            DEFINE INDEX opportunity_game ON TABLE value_opportunities COLUMNS game_id;
            DEFINE INDEX opportunity_type ON TABLE value_opportunities COLUMNS opportunity_type;
            DEFINE INDEX opportunity_confidence ON TABLE value_opportunities COLUMNS confidence;
            DEFINE INDEX opportunity_expires ON TABLE value_opportunities COLUMNS expires_at;
        "#).await?;
        Ok(())
    }

    /// Drop all tables (for testing or reset purposes)
    pub async fn drop_all_tables(db: &Surreal<Client>) -> Result<(), Error> {
        let tables = vec![
            "value_opportunities",
            "line_comparisons", 
            "game_predictions",
            "betting_providers",
            "betting_lines",
            "game_results",
            "player_injuries",
            "team_stats",
            "games",
            "teams",
        ];

        for table in tables {
            db.query(&format!("REMOVE TABLE {}", table)).await?;
        }

        println!("All tables dropped successfully");
        Ok(())
    }

    /// Seed the database with initial data (NFL teams)
    pub async fn seed_nfl_teams(db: &Surreal<Client>) -> Result<(), Error> {
        let teams_data = r#"
            INSERT INTO teams [
                { id: "arizona-cardinals", name: "Arizona Cardinals", abbreviation: "ARI", conference: "NFC", division: "West", created_at: time::now(), updated_at: time::now() },
                { id: "atlanta-falcons", name: "Atlanta Falcons", abbreviation: "ATL", conference: "NFC", division: "South", created_at: time::now(), updated_at: time::now() },
                { id: "baltimore-ravens", name: "Baltimore Ravens", abbreviation: "BAL", conference: "AFC", division: "North", created_at: time::now(), updated_at: time::now() },
                { id: "buffalo-bills", name: "Buffalo Bills", abbreviation: "BUF", conference: "AFC", division: "East", created_at: time::now(), updated_at: time::now() },
                { id: "carolina-panthers", name: "Carolina Panthers", abbreviation: "CAR", conference: "NFC", division: "South", created_at: time::now(), updated_at: time::now() },
                { id: "chicago-bears", name: "Chicago Bears", abbreviation: "CHI", conference: "NFC", division: "North", created_at: time::now(), updated_at: time::now() },
                { id: "cincinnati-bengals", name: "Cincinnati Bengals", abbreviation: "CIN", conference: "AFC", division: "North", created_at: time::now(), updated_at: time::now() },
                { id: "cleveland-browns", name: "Cleveland Browns", abbreviation: "CLE", conference: "AFC", division: "North", created_at: time::now(), updated_at: time::now() },
                { id: "dallas-cowboys", name: "Dallas Cowboys", abbreviation: "DAL", conference: "NFC", division: "East", created_at: time::now(), updated_at: time::now() },
                { id: "denver-broncos", name: "Denver Broncos", abbreviation: "DEN", conference: "AFC", division: "West", created_at: time::now(), updated_at: time::now() },
                { id: "detroit-lions", name: "Detroit Lions", abbreviation: "DET", conference: "NFC", division: "North", created_at: time::now(), updated_at: time::now() },
                { id: "green-bay-packers", name: "Green Bay Packers", abbreviation: "GB", conference: "NFC", division: "North", created_at: time::now(), updated_at: time::now() },
                { id: "houston-texans", name: "Houston Texans", abbreviation: "HOU", conference: "AFC", division: "South", created_at: time::now(), updated_at: time::now() },
                { id: "indianapolis-colts", name: "Indianapolis Colts", abbreviation: "IND", conference: "AFC", division: "South", created_at: time::now(), updated_at: time::now() },
                { id: "jacksonville-jaguars", name: "Jacksonville Jaguars", abbreviation: "JAX", conference: "AFC", division: "South", created_at: time::now(), updated_at: time::now() },
                { id: "kansas-city-chiefs", name: "Kansas City Chiefs", abbreviation: "KC", conference: "AFC", division: "West", created_at: time::now(), updated_at: time::now() },
                { id: "las-vegas-raiders", name: "Las Vegas Raiders", abbreviation: "LV", conference: "AFC", division: "West", created_at: time::now(), updated_at: time::now() },
                { id: "los-angeles-chargers", name: "Los Angeles Chargers", abbreviation: "LAC", conference: "AFC", division: "West", created_at: time::now(), updated_at: time::now() },
                { id: "los-angeles-rams", name: "Los Angeles Rams", abbreviation: "LAR", conference: "NFC", division: "West", created_at: time::now(), updated_at: time::now() },
                { id: "miami-dolphins", name: "Miami Dolphins", abbreviation: "MIA", conference: "AFC", division: "East", created_at: time::now(), updated_at: time::now() },
                { id: "minnesota-vikings", name: "Minnesota Vikings", abbreviation: "MIN", conference: "NFC", division: "North", created_at: time::now(), updated_at: time::now() },
                { id: "new-england-patriots", name: "New England Patriots", abbreviation: "NE", conference: "AFC", division: "East", created_at: time::now(), updated_at: time::now() },
                { id: "new-orleans-saints", name: "New Orleans Saints", abbreviation: "NO", conference: "NFC", division: "South", created_at: time::now(), updated_at: time::now() },
                { id: "new-york-giants", name: "New York Giants", abbreviation: "NYG", conference: "NFC", division: "East", created_at: time::now(), updated_at: time::now() },
                { id: "new-york-jets", name: "New York Jets", abbreviation: "NYJ", conference: "AFC", division: "East", created_at: time::now(), updated_at: time::now() },
                { id: "philadelphia-eagles", name: "Philadelphia Eagles", abbreviation: "PHI", conference: "NFC", division: "East", created_at: time::now(), updated_at: time::now() },
                { id: "pittsburgh-steelers", name: "Pittsburgh Steelers", abbreviation: "PIT", conference: "AFC", division: "North", created_at: time::now(), updated_at: time::now() },
                { id: "san-francisco-49ers", name: "San Francisco 49ers", abbreviation: "SF", conference: "NFC", division: "West", created_at: time::now(), updated_at: time::now() },
                { id: "seattle-seahawks", name: "Seattle Seahawks", abbreviation: "SEA", conference: "NFC", division: "West", created_at: time::now(), updated_at: time::now() },
                { id: "tampa-bay-buccaneers", name: "Tampa Bay Buccaneers", abbreviation: "TB", conference: "NFC", division: "South", created_at: time::now(), updated_at: time::now() },
                { id: "tennessee-titans", name: "Tennessee Titans", abbreviation: "TEN", conference: "AFC", division: "South", created_at: time::now(), updated_at: time::now() },
                { id: "washington-commanders", name: "Washington Commanders", abbreviation: "WAS", conference: "NFC", division: "East", created_at: time::now(), updated_at: time::now() }
            ];
        "#;

        db.query(teams_data).await?;
        println!("NFL teams seeded successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connect;

    #[tokio::test]
    async fn test_schema_initialization() {
        let db = connect().await.expect("Failed to connect to database");
        
        // Clean up any existing tables
        let _ = Schema::drop_all_tables(&db).await;
        
        // Initialize schema
        let result = Schema::initialize(&db).await;
        assert!(result.is_ok());
        
        // Verify tables exist by trying to query them
        let teams_result = db.query("SELECT * FROM teams LIMIT 1").await;
        assert!(teams_result.is_ok());
        
        let games_result = db.query("SELECT * FROM games LIMIT 1").await;
        assert!(games_result.is_ok());
        
        // Clean up
        let _ = Schema::drop_all_tables(&db).await;
    }

    #[tokio::test]
    async fn test_nfl_teams_seeding() {
        let db = connect().await.expect("Failed to connect to database");
        
        // Clean up and initialize
        let _ = Schema::drop_all_tables(&db).await;
        Schema::initialize(&db).await.expect("Failed to initialize schema");
        
        // Seed teams
        let result = Schema::seed_nfl_teams(&db).await;
        assert!(result.is_ok());
        
        // Verify teams were inserted
        let teams: Vec<serde_json::Value> = db.query("SELECT * FROM teams").await
            .expect("Failed to query teams")
            .take(0)
            .expect("Failed to take result");
        
        assert_eq!(teams.len(), 32); // Should have 32 NFL teams
        
        // Verify specific team exists
        let chiefs: Vec<serde_json::Value> = db.query("SELECT * FROM teams WHERE abbreviation = 'KC'").await
            .expect("Failed to query Chiefs")
            .take(0)
            .expect("Failed to take result");
        
        assert_eq!(chiefs.len(), 1);
        
        // Clean up
        let _ = Schema::drop_all_tables(&db).await;
    }

    #[tokio::test]
    async fn test_table_drop() {
        let db = connect().await.expect("Failed to connect to database");
        
        // Initialize schema
        Schema::initialize(&db).await.expect("Failed to initialize schema");
        
        // Drop all tables
        let result = Schema::drop_all_tables(&db).await;
        assert!(result.is_ok());
        
        // Verify tables are dropped by trying to query them (should fail)
        let teams_result = db.query("SELECT * FROM teams LIMIT 1").await;
        assert!(teams_result.is_err());
    }
}