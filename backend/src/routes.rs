use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::{State, fairing::{Fairing, Info, Kind}};

use crate::db::{error::Error, DatabaseManager};
use share::models::{Game, Team, BettingLine, GamePrediction};

// Rocket fairing for simplified database initialization
pub struct DatabaseFairing;

#[rocket::async_trait]
impl Fairing for DatabaseFairing {
    fn info(&self) -> Info {
        Info {
            name: "Database Connection",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: rocket::Rocket<rocket::Build>) -> rocket::fairing::Result {
        match DatabaseManager::new().await {
            Ok(db_manager) => {
                // Database is ready - collections will be created automatically when data is inserted
                println!("Database connection established successfully");
                Ok(rocket.manage(db_manager))
            },
            Err(e) => {
                eprintln!("Failed to connect to database: {:?}", e);
                Err(rocket)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiPayload {
    Game(Game),
    Team(Team),
    BettingLine(BettingLine),
    GamePrediction(GamePrediction),
}

// ===== TEAM ROUTES =====

#[post("/teams", data = "<team>")]
pub async fn create_team(
    team: Json<Team>,
    db: &State<DatabaseManager>,
) -> Result<Json<String>, Error> {
    let team_data = team.into_inner();
    
    // Validate the team data at struct level
    let validated_team = team_data.validate_and_create()
        .map_err(|_| Error::EntryExists)?; // Reusing existing error for validation
    
    let record_id = db.store("teams", validated_team).await?;
    Ok(Json(record_id.to_string()))
}

#[get("/teams/<id>")]
pub async fn get_team(
    id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<Option<Team>>, Error> {
    let team = db.get("teams", id).await?;
    Ok(Json(team))
}

#[get("/teams")]
pub async fn get_all_teams(
    db: &State<DatabaseManager>
) -> Result<Json<Vec<Team>>, Error> {
    let teams = db.get_all("teams").await?;
    Ok(Json(teams))
}

#[put("/teams/<id>", data = "<team>")]
pub async fn update_team(
    id: &str,
    team: Json<Team>,
    db: &State<DatabaseManager>,
) -> Result<Json<Option<Team>>, Error> {
    let team_data = team.into_inner();
    let result = db.update("teams", id, team_data).await?;
    Ok(Json(result))
}

#[delete("/teams/<id>")]
pub async fn delete_team(
    id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<bool>, Error> {
    let _: Option<Team> = db.delete("teams", id).await?;
    Ok(Json(true))
}

// ===== GAME ROUTES =====

#[post("/games", data = "<game>")]
pub async fn create_game(
    game: Json<Game>,
    db: &State<DatabaseManager>,
) -> Result<Json<String>, Error> {
    let game_data = game.into_inner();
    let record_id = db.store("games", game_data).await?;
    Ok(Json(record_id.to_string()))
}

#[get("/games/<id>")]
pub async fn get_game(
    id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<Option<Game>>, Error> {
    let game = db.get("games", id).await?;
    Ok(Json(game))
}

#[get("/games")]
pub async fn get_all_games(
    db: &State<DatabaseManager>
) -> Result<Json<Vec<Game>>, Error> {
    let games = db.get_all("games").await?;
    Ok(Json(games))
}

#[get("/games/week/<week>/season/<season>")]
pub async fn get_games_by_week(
    week: u8,
    season: u16,
    db: &State<DatabaseManager>
) -> Result<Json<Vec<Game>>, Error> {
    let mut response = db.db.query("SELECT * FROM games WHERE week = $week AND season = $season")
        .bind(("week", week))
        .bind(("season", season))
        .await?;
    
    let games: Vec<Game> = response.take(0)?;
    Ok(Json(games))
}

#[put("/games/<id>", data = "<game>")]
pub async fn update_game(
    id: &str,
    game: Json<Game>,
    db: &State<DatabaseManager>,
) -> Result<Json<Option<Game>>, Error> {
    let game_data = game.into_inner();
    let result = db.update("games", id, game_data).await?;
    Ok(Json(result))
}

#[delete("/games/<id>")]
pub async fn delete_game(
    id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<bool>, Error> {
    let _: Option<Game> = db.delete("games", id).await?;
    Ok(Json(true))
}

// ===== BETTING LINE ROUTES =====

#[post("/betting-lines", data = "<line>")]
pub async fn create_betting_line(
    line: Json<BettingLine>,
    db: &State<DatabaseManager>,
) -> Result<Json<String>, Error> {
    let line_data = line.into_inner();
    let record_id = db.store("betting_lines", line_data).await?;
    Ok(Json(record_id.to_string()))
}

#[get("/betting-lines/<id>")]
pub async fn get_betting_line(
    id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<Option<BettingLine>>, Error> {
    let line = db.get("betting_lines", id).await?;
    Ok(Json(line))
}

#[get("/betting-lines/game/<game_id>")]
pub async fn get_betting_lines_for_game(
    game_id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<Vec<BettingLine>>, Error> {
    let game_id_owned = game_id.to_string();
    let mut response = db.db
        .query("SELECT * FROM betting_lines WHERE game_id = $game_id AND is_active = true")
        .bind(("game_id", game_id_owned))
        .await?;
    
    let lines: Vec<BettingLine> = response.take(0)?;
    Ok(Json(lines))
}

// ===== PREDICTION ROUTES =====

#[post("/predictions", data = "<prediction>")]
pub async fn create_prediction(
    prediction: Json<GamePrediction>,
    db: &State<DatabaseManager>,
) -> Result<Json<String>, Error> {
    let prediction_data = prediction.into_inner();
    let record_id = db.store("predictions", prediction_data).await?;
    Ok(Json(record_id.to_string()))
}

#[get("/predictions/<id>")]
pub async fn get_prediction(
    id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<Option<GamePrediction>>, Error> {
    let prediction = db.get("predictions", id).await?;
    Ok(Json(prediction))
}

#[get("/predictions/game/<game_id>")]
pub async fn get_prediction_for_game(
    game_id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<Option<GamePrediction>>, Error> {
    let game_id_owned = game_id.to_string();
    let mut response = db.db
        .query("SELECT * FROM predictions WHERE game_id = $game_id ORDER BY generated_at DESC LIMIT 1")
        .bind(("game_id", game_id_owned))
        .await?;
    
    let predictions: Vec<GamePrediction> = response.take(0)?;
    Ok(Json(predictions.into_iter().next()))
}
