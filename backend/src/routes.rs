use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;
use tokio::time::sleep;

use rocket::serde::json::Json;
use rocket::{State, fairing::{Fairing, Info, Kind}};

use crate::db::error::Error;
use share::models::{Game, Team, BettingLine, GamePrediction};

// Database manager that will be managed by Rocket
pub struct DatabaseManager {
    pub db: Surreal<Client>,
}

impl DatabaseManager {
    pub async fn new() -> Result<Self, surrealdb::Error> {
        let db = Surreal::init();
        
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let namespace = env::var("DATABASE_NS").expect("DATABASE_NS must be set");
        let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

        // Connect with retry logic
        connect_with_retry(&db, &database_url).await?;

        // Authenticate
        db.signin(surrealdb::opt::auth::Root {
            username: "root",
            password: "root",
        })
        .await?;

        // Switch to the desired namespace and database
        db.use_ns(namespace).use_db(database_name).await?;

        eprintln!("Connected to SurrealDB!");

        Ok(DatabaseManager { db })
    }
}

// Rocket fairing for database initialization
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
            Ok(db_manager) => Ok(rocket.manage(db_manager)),
            Err(e) => {
                eprintln!("Failed to connect to database: {:?}", e);
                Err(rocket)
            }
        }
    }
}

// LOGIN INFO
const MAX_RETRIES: i8 = 5;
const DELAY: Duration = Duration::from_secs(5);

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiPayload {
    Game(Game),
    Team(Team),
    BettingLine(BettingLine),
    GamePrediction(GamePrediction),
}

async fn connect_with_retry(db: &Surreal<Client>, database_url: &str) -> Result<(), surrealdb::Error> {
    let mut failures = 0;

    loop {
        let res = db.connect::<Ws>(database_url).await;
        match res {
            Ok(_) => break,
            Err(e) => {
                eprintln!("Database connection attempt {} failed: {:?}", failures + 1, e);

                if failures >= MAX_RETRIES {
                    return Err(e);
                }
                failures += 1;

                sleep(DELAY).await;
                continue;
            }
        }
    }
    Ok(())
}

// ===== TEAM ROUTES =====

#[post("/teams", data = "<team>")]
pub async fn create_team(
    team: Json<Team>,
    db: &State<DatabaseManager>,
) -> Result<Json<Team>, Error> {
    let team_data = team.into_inner();
    let result: Option<Team> = db.db.create("teams").content(team_data).await?;
    match result {
        Some(team) => Ok(Json(team)),
        None => Err(Error::Db), // Handle the case where creation fails
    }
}

#[get("/teams/<id>")]
pub async fn get_team(
    id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<Option<Team>>, Error> {
    let team: Option<Team> = db.db.select(("teams", id)).await?;
    Ok(Json(team))
}

#[get("/teams")]
pub async fn get_all_teams(
    db: &State<DatabaseManager>
) -> Result<Json<Vec<Team>>, Error> {
    let teams: Vec<Team> = db.db.select("teams").await?;
    Ok(Json(teams))
}

#[put("/teams/<id>", data = "<team>")]
pub async fn update_team(
    id: &str,
    team: Json<Team>,
    db: &State<DatabaseManager>,
) -> Result<Json<Option<Team>>, Error> {
    let team_data = team.into_inner();
    let result: Option<Team> = db.db.update(("teams", id)).content(team_data).await?;
    Ok(Json(result))
}

#[delete("/teams/<id>")]
pub async fn delete_team(
    id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<bool>, Error> {
    let _: Option<Team> = db.db.delete(("teams", id)).await?;
    Ok(Json(true))
}

// ===== GAME ROUTES =====

#[post("/games", data = "<game>")]
pub async fn create_game(
    game: Json<Game>,
    db: &State<DatabaseManager>,
) -> Result<Json<Game>, Error> {
    let game_data = game.into_inner();
    let result: Option<Game> = db.db.create("games").content(game_data).await?;
    match result {
        Some(game) => Ok(Json(game)),
        None => Err(Error::Db),
    }
}

#[get("/games/<id>")]
pub async fn get_game(
    id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<Option<Game>>, Error> {
    let game: Option<Game> = db.db.select(("games", id)).await?;
    Ok(Json(game))
}

#[get("/games")]
pub async fn get_all_games(
    db: &State<DatabaseManager>
) -> Result<Json<Vec<Game>>, Error> {
    let games: Vec<Game> = db.db.select("games").await?;
    Ok(Json(games))
}

#[get("/games/week/<week>/season/<season>")]
pub async fn get_games_by_week(
    week: u8,
    season: u16,
    db: &State<DatabaseManager>
) -> Result<Json<Vec<Game>>, Error> {
    let games: Vec<Game> = db.db
        .query("SELECT * FROM games WHERE week = $week AND season = $season")
        .bind(("week", week))
        .bind(("season", season))
        .await?
        .take(0)?;
    Ok(Json(games))
}

#[put("/games/<id>", data = "<game>")]
pub async fn update_game(
    id: &str,
    game: Json<Game>,
    db: &State<DatabaseManager>,
) -> Result<Json<Option<Game>>, Error> {
    let game_data = game.into_inner();
    let result: Option<Game> = db.db.update(("games", id)).content(game_data).await?;
    Ok(Json(result))
}


#[delete("/games/<id>")]
pub async fn delete_game(
    id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<bool>, Error> {
    let _: Option<Game> = db.db.delete(("games", id)).await?;
    Ok(Json(true))
}

// ===== BETTING LINE ROUTES =====

#[post("/betting-lines", data = "<line>")]
pub async fn create_betting_line(
    line: Json<BettingLine>,
    db: &State<DatabaseManager>,
) -> Result<Json<BettingLine>, Error> {
    let line_data = line.into_inner();
    let result: Option<BettingLine> = db.db.create("betting_lines").content(line_data).await?;
    match result {
        Some(line) => Ok(Json(line)),
        None => Err(Error::Db),
    }
}

#[get("/betting-lines/<id>")]
pub async fn get_betting_line(
    id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<Option<BettingLine>>, Error> {
    let line: Option<BettingLine> = db.db.select(("betting_lines", id)).await?;
    Ok(Json(line))
}

#[get("/betting-lines/game/<game_id>")]
pub async fn get_betting_lines_for_game(
    game_id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<Vec<BettingLine>>, Error> {
    let game_id_owned = game_id.to_string();
    let lines: Vec<BettingLine> = db.db
        .query("SELECT * FROM betting_lines WHERE game_id = $game_id AND is_active = true")
        .bind(("game_id", game_id_owned))
        .await?
        .take(0)?;
    Ok(Json(lines))
}

// ===== PREDICTION ROUTES =====

#[post("/predictions", data = "<prediction>")]
pub async fn create_prediction(
    prediction: Json<GamePrediction>,
    db: &State<DatabaseManager>,
) -> Result<Json<GamePrediction>, Error> {
    let prediction_data = prediction.into_inner();
    let result: Option<GamePrediction> = db.db.create("game_predictions").content(prediction_data).await?;
    match result {
        Some(prediction) => Ok(Json(prediction)),
        None => Err(Error::Db),
    }
}

#[get("/predictions/<id>")]
pub async fn get_prediction(
    id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<Option<GamePrediction>>, Error> {
    let prediction: Option<GamePrediction> = db.db.select(("game_predictions", id)).await?;
    Ok(Json(prediction))
}

#[get("/predictions/game/<game_id>")]
pub async fn get_prediction_for_game(
    game_id: &str,
    db: &State<DatabaseManager>
) -> Result<Json<Option<GamePrediction>>, Error> {
    let game_id_owned = game_id.to_string();
    let predictions: Vec<GamePrediction> = db.db
        .query("SELECT * FROM game_predictions WHERE game_id = $game_id ORDER BY generated_at DESC LIMIT 1")
        .bind(("game_id", game_id_owned))
        .await?
        .take(0)?;
    
    Ok(Json(predictions.into_iter().next()))
}
