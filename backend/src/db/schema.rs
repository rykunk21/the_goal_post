use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::db::error::Error;

/// Database schema management for the NFL prediction dashboard
pub struct Schema;

impl Schema {
    /// Initialize the database schema
    pub async fn initialize(db: &Surreal<Client>) -> Result<(), Error> {
        println!("Initializing database schema...");
        
        // Create teams table
        db.query(r#"
            DEFINE TABLE teams SCHEMAFULL;
            DEFINE FIELD id ON teams TYPE string;
            DEFINE FIELD name ON teams TYPE string;
            DEFINE FIELD abbreviation ON teams TYPE string;
            DEFINE FIELD city ON teams TYPE string;
            DEFINE FIELD conference ON teams TYPE string;
            DEFINE FIELD division ON teams TYPE string;
            DEFINE FIELD stats ON teams TYPE object;
            DEFINE FIELD created_at ON teams TYPE datetime DEFAULT time::now();
            DEFINE INDEX teams_abbreviation ON teams COLUMNS abbreviation UNIQUE;
        "#).await?;
        
        // Create games table
        db.query(r#"
            DEFINE TABLE games SCHEMAFULL;
            DEFINE FIELD id ON games TYPE string;
            DEFINE FIELD week ON games TYPE int;
            DEFINE FIELD season ON games TYPE int;
            DEFINE FIELD game_time ON games TYPE datetime;
            DEFINE FIELD home_team_id ON games TYPE string;
            DEFINE FIELD away_team_id ON games TYPE string;
            DEFINE FIELD status ON games TYPE string;
            DEFINE FIELD created_at ON games TYPE datetime DEFAULT time::now();
        "#).await?;
        
        // Create predictions table
        db.query(r#"
            DEFINE TABLE predictions SCHEMAFULL;
            DEFINE FIELD id ON predictions TYPE string;
            DEFINE FIELD game_id ON predictions TYPE string;
            DEFINE FIELD spread_prediction ON predictions TYPE float;
            DEFINE FIELD total_prediction ON predictions TYPE float;
            DEFINE FIELD home_score_distribution ON predictions TYPE object;
            DEFINE FIELD away_score_distribution ON predictions TYPE object;
            DEFINE FIELD confidence_interval ON predictions TYPE object;
            DEFINE FIELD generated_at ON predictions TYPE datetime DEFAULT time::now();
        "#).await?;
        
        // Create betting_lines table
        db.query(r#"
            DEFINE TABLE betting_lines SCHEMAFULL;
            DEFINE FIELD id ON betting_lines TYPE string;
            DEFINE FIELD game_id ON betting_lines TYPE string;
            DEFINE FIELD provider ON betting_lines TYPE string;
            DEFINE FIELD spread ON betting_lines TYPE float;
            DEFINE FIELD total ON betting_lines TYPE float;
            DEFINE FIELD moneyline_home ON betting_lines TYPE int;
            DEFINE FIELD moneyline_away ON betting_lines TYPE int;
            DEFINE FIELD timestamp ON betting_lines TYPE datetime DEFAULT time::now();
            DEFINE FIELD is_active ON betting_lines TYPE bool DEFAULT true;
        "#).await?;
        
        // Create value_opportunities table
        db.query(r#"
            DEFINE TABLE value_opportunities SCHEMAFULL;
            DEFINE FIELD id ON value_opportunities TYPE string;
            DEFINE FIELD game_id ON value_opportunities TYPE string;
            DEFINE FIELD opportunity_type ON value_opportunities TYPE string;
            DEFINE FIELD confidence ON value_opportunities TYPE float;
            DEFINE FIELD expected_value ON value_opportunities TYPE float;
            DEFINE FIELD recommendation ON value_opportunities TYPE string;
            DEFINE FIELD betting_line_id ON value_opportunities TYPE string;
            DEFINE FIELD created_at ON value_opportunities TYPE datetime DEFAULT time::now();
            DEFINE FIELD expires_at ON value_opportunities TYPE datetime;
        "#).await?;
        
        println!("Database schema initialized successfully");
        Ok(())
    }
    
    /// Seed NFL teams data
    pub async fn seed_nfl_teams(db: &Surreal<Client>) -> Result<(), Error> {
        println!("Seeding NFL teams...");
        
        let teams_data = r#"
            CREATE teams:afc_east_buf SET {
                id: "afc_east_buf",
                name: "Buffalo Bills",
                abbreviation: "BUF",
                city: "Buffalo",
                conference: "AFC",
                division: "East",
                stats: {
                    wins: 0,
                    losses: 0,
                    ties: 0,
                    offensive_rating: 85.0,
                    defensive_rating: 80.0
                }
            };
            CREATE teams:afc_east_mia SET {
                id: "afc_east_mia",
                name: "Miami Dolphins",
                abbreviation: "MIA",
                city: "Miami",
                conference: "AFC",
                division: "East",
                stats: {
                    wins: 0,
                    losses: 0,
                    ties: 0,
                    offensive_rating: 82.0,
                    defensive_rating: 78.0
                }
            };
            CREATE teams:afc_east_ne SET {
                id: "afc_east_ne",
                name: "New England Patriots",
                abbreviation: "NE",
                city: "New England",
                conference: "AFC",
                division: "East",
                stats: {
                    wins: 0,
                    losses: 0,
                    ties: 0,
                    offensive_rating: 75.0,
                    defensive_rating: 82.0
                }
            };
            CREATE teams:afc_east_nyj SET {
                id: "afc_east_nyj",
                name: "New York Jets",
                abbreviation: "NYJ",
                city: "New York",
                conference: "AFC",
                division: "East",
                stats: {
                    wins: 0,
                    losses: 0,
                    ties: 0,
                    offensive_rating: 78.0,
                    defensive_rating: 85.0
                }
            };
        "#;
        
        db.query(teams_data).await?;
        
        // Add more teams (abbreviated for brevity - in real implementation, add all 32 teams)
        let more_teams = r#"
            CREATE teams:nfc_south_atl SET {
                id: "nfc_south_atl",
                name: "Atlanta Falcons",
                abbreviation: "ATL",
                city: "Atlanta",
                conference: "NFC",
                division: "South",
                stats: {
                    wins: 0,
                    losses: 0,
                    ties: 0,
                    offensive_rating: 80.0,
                    defensive_rating: 75.0
                }
            };
            CREATE teams:nfc_south_car SET {
                id: "nfc_south_car",
                name: "Carolina Panthers",
                abbreviation: "CAR",
                city: "Carolina",
                conference: "NFC",
                division: "South",
                stats: {
                    wins: 0,
                    losses: 0,
                    ties: 0,
                    offensive_rating: 70.0,
                    defensive_rating: 72.0
                }
            };
        "#;
        
        db.query(more_teams).await?;
        
        println!("NFL teams seeded successfully");
        Ok(())
    }
    
    /// Drop all tables (for testing/reset purposes)
    pub async fn drop_all_tables(db: &Surreal<Client>) -> Result<(), Error> {
        println!("Dropping all tables...");
        
        let drop_queries = r#"
            REMOVE TABLE IF EXISTS value_opportunities;
            REMOVE TABLE IF EXISTS betting_lines;
            REMOVE TABLE IF EXISTS predictions;
            REMOVE TABLE IF EXISTS games;
            REMOVE TABLE IF EXISTS teams;
            REMOVE TABLE IF EXISTS migration_history;
        "#;
        
        db.query(drop_queries).await?;
        
        println!("All tables dropped successfully");
        Ok(())
    }
}