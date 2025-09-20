use yew::prelude::*;
use web_sys::{HtmlInputElement, FileReader};
use share::models::*;
use std::collections::HashMap;
use chrono::{DateTime, Utc, NaiveDate, NaiveTime};
use wasm_bindgen::{JsCast, closure::Closure};

use super::dashboard::GameWithPredictionAndLines;

#[derive(Properties, PartialEq)]
pub struct MockDataFormProps {
    pub on_submit: Callback<GameWithPredictionAndLines>,
    pub on_bulk_submit: Callback<Vec<GameWithPredictionAndLines>>,
}

// Team abbreviation to full name mapping
fn create_nfl_prediction_games() -> Vec<GameWithPredictionAndLines> {
    // This data is based on our probability analysis from temp/probability_analysis.py
    let games_data: Vec<(&str, &str, &str, &str, f64, f64, f64, f64, f64, &str, f64)> = vec![
        ("Atlanta Falcons", "ATL", "Carolina Panthers", "CAR", 26.7, 16.5, 0.34, 5.5, 45.0, "CAR +5.5", 17.1),
        ("Green Bay Packers", "GB", "Cleveland Browns", "CLE", 28.9, 10.3, 0.62, 8.5, 45.0, "CLE +8.5", 11.9),
        ("Houston Texans", "HOU", "Jacksonville Jaguars", "JAX", 21.6, 23.4, 0.06, -1.5, 45.0, "JAX -1.5", -8.2),
        ("Cincinnati Bengals", "CIN", "Minnesota Vikings", "MIN", 19.2, 25.2, 0.2, -3.0, 45.0, "MIN -3.0", -11.3),
        ("Pittsburgh Steelers", "PIT", "New England Patriots", "NE", 23.9, 20.9, 0.1, 1.5, 45.0, "NE +1.5", 6.2),
        ("Los Angeles Rams", "LA", "Philadelphia Eagles", "PHI", 18.1, 25.9, 0.26, -3.5, 45.0, "PHI -3.5", -11.3),
        ("New York Jets", "NYJ", "Tampa Bay Buccaneers", "TB", 13.1, 28.1, 0.5, -6.5, 45.0, "TB -6.5", -12.8),
        ("Indianapolis Colts", "IND", "Tennessee Titans", "TEN", 26.3, 17.3, 0.3, 4.5, 45.0, "TEN +4.5", 14.6),
        ("Las Vegas Raiders", "LV", "Washington Commanders", "WAS", 19.6, 25.0, 0.18, -2.5, 45.0, "LV +2.5", 9.1),
        ("Denver Broncos", "DEN", "Los Angeles Chargers", "LAC", 19.6, 25.0, 0.18, -3.0, 45.0, "DEN +3.0", 12.3),
        ("New Orleans Saints", "NO", "Seattle Seahawks", "SEA", 11.8, 28.6, 0.56, -7.5, 45.0, "NO +7.5", 12.7),
        ("Dallas Cowboys", "DAL", "Chicago Bears", "CHI", 23.1, 21.9, 0.04, 1.0, 45.0, "CHI +1.0", 5.5),
        ("Arizona Cardinals", "ARI", "San Francisco 49ers", "SF", 20.9, 23.9, 0.1, 0.0, 45.0, "SF EVEN", 5.0),
        ("Kansas City Chiefs", "KC", "New York Giants", "NYG", 28.0, 13.6, 0.48, 6.5, 45.0, "NYG +6.5", 13.8),
        ("Detroit Lions", "DET", "Baltimore Ravens", "BAL", 16.1, 26.9, 0.36, -4.5, 45.0, "BAL -4.5", -11.6),
    ];

    games_data.into_iter().map(|(away_name, away_abbr, home_name, home_abbr, away_score, home_score, confidence, spread, total, bet_rec, value_pct)| {
        let game_id = format!("nfl_week3_{}_{}", away_abbr, home_abbr);
        let line_id = format!("line_{}", game_id);
        
        // Create value opportunity if there's significant value
        let value_opportunities = if value_pct.abs() >= 5.0 {
            vec![ValueOpportunity::new(
                game_id.clone(),
                OpportunityType::SpreadValue,
                confidence.max(0.6), // Ensure reasonable confidence
                value_pct / 100.0, // Convert percentage to decimal
                bet_rec.to_string(),
                line_id.clone(),
            )]
        } else {
            vec![]
        };

        GameWithPredictionAndLines {
            game: Game {
                id: game_id.clone(),
                week: 3,
                season: 2025,
                game_time: Utc::now() + chrono::Duration::days(1), // Tomorrow
                home_team: Team {
                    id: format!("team_{}", home_abbr),
                    name: home_name.to_string(),
                    abbreviation: home_abbr.to_string(),
                    conference: Some("NFC".to_string()),
                    division: Some("North".to_string()),
                    stats: TeamStats {
                        offensive_rating: 80.0,
                        defensive_rating: 75.0,
                        points_per_game: 24.0,
                        points_allowed_per_game: 20.0,
                        yards_per_game: 350.0,
                        yards_allowed_per_game: 320.0,
                        turnover_differential: 2,
                        recent_form: vec![],
                        injury_report: vec![],
                        season: 2025,
                        games_played: 2,
                        wins: 1,
                        losses: 1,
                        ties: 0,
                        last_updated: Utc::now(),
                    },
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
                away_team: Team {
                    id: format!("team_{}", away_abbr),
                    name: away_name.to_string(),
                    abbreviation: away_abbr.to_string(),
                    conference: Some("AFC".to_string()),
                    division: Some("North".to_string()),
                    stats: TeamStats {
                        offensive_rating: 78.0,
                        defensive_rating: 77.0,
                        points_per_game: 22.0,
                        points_allowed_per_game: 21.0,
                        yards_per_game: 340.0,
                        yards_allowed_per_game: 330.0,
                        turnover_differential: 0,
                        recent_form: vec![],
                        injury_report: vec![],
                        season: 2025,
                        games_played: 2,
                        wins: 1,
                        losses: 1,
                        ties: 0,
                        last_updated: Utc::now(),
                    },
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
                status: GameStatus::Scheduled,
                home_score: None,
                away_score: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            prediction: Some(GamePrediction {
                id: format!("pred_{}", game_id),
                game_id: game_id.clone(),
                home_score_distribution: ProbabilityDistribution {
                    mean: home_score,
                    std_dev: 7.0,
                    samples: vec![home_score - 3.0, home_score, home_score + 3.0],
                    percentiles: std::collections::HashMap::new(),
                },
                away_score_distribution: ProbabilityDistribution {
                    mean: away_score,
                    std_dev: 7.0,
                    samples: vec![away_score - 3.0, away_score, away_score + 3.0],
                    percentiles: std::collections::HashMap::new(),
                },
                spread_prediction: home_score - away_score,
                total_prediction: home_score + away_score,
                confidence_interval: ConfidenceInterval {
                    lower_bound: (home_score + away_score) - 5.0,
                    upper_bound: (home_score + away_score) + 5.0,
                    confidence_level: 0.95,
                },
                generated_at: Utc::now(),
            }),
            betting_lines: vec![BettingLine::new(
                game_id.clone(),
                "Probability Analysis".to_string(),
                spread,
                total,
                -110,
                -110,
            )],
            value_opportunities,
        }
    }).collect()
}

fn get_team_name(abbr: &str) -> String {
    match abbr {
        "ARI" => "Arizona Cardinals".to_string(),
        "ATL" => "Atlanta Falcons".to_string(),
        "BAL" => "Baltimore Ravens".to_string(),
        "BUF" => "Buffalo Bills".to_string(),
        "CAR" => "Carolina Panthers".to_string(),
        "CHI" => "Chicago Bears".to_string(),
        "CIN" => "Cincinnati Bengals".to_string(),
        "CLE" => "Cleveland Browns".to_string(),
        "DAL" => "Dallas Cowboys".to_string(),
        "DEN" => "Denver Broncos".to_string(),
        "DET" => "Detroit Lions".to_string(),
        "GB" => "Green Bay Packers".to_string(),
        "HOU" => "Houston Texans".to_string(),
        "IND" => "Indianapolis Colts".to_string(),
        "JAX" => "Jacksonville Jaguars".to_string(),
        "KC" => "Kansas City Chiefs".to_string(),
        "LV" => "Las Vegas Raiders".to_string(),
        "LAC" => "Los Angeles Chargers".to_string(),
        "LA" => "Los Angeles Rams".to_string(),
        "MIA" => "Miami Dolphins".to_string(),
        "MIN" => "Minnesota Vikings".to_string(),
        "NE" => "New England Patriots".to_string(),
        "NO" => "New Orleans Saints".to_string(),
        "NYG" => "New York Giants".to_string(),
        "NYJ" => "New York Jets".to_string(),
        "PHI" => "Philadelphia Eagles".to_string(),
        "PIT" => "Pittsburgh Steelers".to_string(),
        "SEA" => "Seattle Seahawks".to_string(),
        "SF" => "San Francisco 49ers".to_string(),
        "TB" => "Tampa Bay Buccaneers".to_string(),
        "TEN" => "Tennessee Titans".to_string(),
        "WAS" => "Washington Commanders".to_string(),
        _ => format!("Unknown Team ({})", abbr),
    }
}

#[derive(Clone, Debug)]
struct CsvGameData {
    week: u8,
    date: String,
    time: String,
    away_team: String,
    home_team: String,
    predicted_away_score: f64,
    predicted_home_score: f64,
    confidence: f64,
    market_spread: f64,
    total: f64,
}

fn parse_csv_data(csv_content: &str) -> Result<Vec<CsvGameData>, String> {
    let mut games = Vec::new();
    let lines: Vec<&str> = csv_content.lines().collect();
    
    // Skip header line
    for line in lines.iter().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 10 {
            continue;
        }
        
        // Parse market spread - CSV already uses our internal convention
        // CSV: -5.5 means home team is 5.5-point underdog
        // CSV: +3.0 means home team is 3.0-point favorite
        let internal_spread: f64 = fields[8].parse().map_err(|_| "Invalid spread")?;
        
        let game_data = CsvGameData {
            week: fields[0].parse().map_err(|_| "Invalid week")?,
            date: fields[1].to_string(),
            time: fields[2].to_string(),
            away_team: fields[3].to_string(),
            home_team: fields[4].to_string(),
            predicted_away_score: fields[5].parse().map_err(|_| "Invalid away score")?,
            predicted_home_score: fields[6].parse().map_err(|_| "Invalid home score")?,
            confidence: fields[7].parse().map_err(|_| "Invalid confidence")?,
            market_spread: internal_spread,
            total: fields[9].parse().map_err(|_| "Invalid total")?,
        };
        
        games.push(game_data);
    }
    
    Ok(games)
}

fn csv_to_game_data(csv_game: CsvGameData) -> GameWithPredictionAndLines {
    let home_team_name = get_team_name(&csv_game.home_team);
    let away_team_name = get_team_name(&csv_game.away_team);
    
    // Parse date and time
    let game_time = match NaiveDate::parse_from_str(&csv_game.date, "%Y-%m-%d") {
        Ok(date) => {
            let time = NaiveTime::parse_from_str(&csv_game.time, "%H:%M").unwrap_or(NaiveTime::from_hms_opt(13, 0, 0).unwrap());
            DateTime::from_naive_utc_and_offset(date.and_time(time), Utc)
        },
        Err(_) => Utc::now() + chrono::Duration::days(7), // Default to next week
    };
    
    create_mock_game_data(
        home_team_name,
        csv_game.home_team,
        away_team_name,
        csv_game.away_team,
        8, // Default wins
        3, // Default losses
        7, // Default wins
        4, // Default losses
        csv_game.week,
        2025, // Season from CSV
        csv_game.predicted_home_score,
        csv_game.predicted_away_score,
        csv_game.confidence,
        csv_game.market_spread,
        csv_game.total,
        "CSV Data".to_string(),
    )
}

#[function_component(MockDataForm)]
pub fn mock_data_form(props: &MockDataFormProps) -> Html {
    let home_team_name = use_state(|| "Kansas City Chiefs".to_string());
    let home_team_abbr = use_state(|| "KC".to_string());
    let away_team_name = use_state(|| "Buffalo Bills".to_string());
    let away_team_abbr = use_state(|| "BUF".to_string());
    
    let home_wins = use_state(|| 8u8);
    let home_losses = use_state(|| 3u8);
    let away_wins = use_state(|| 7u8);
    let away_losses = use_state(|| 4u8);
    
    let game_week = use_state(|| 12u8);
    let game_season = use_state(|| 2024u16);
    
    // Prediction data
    let predicted_home_score = use_state(|| 27.5f64);
    let predicted_away_score = use_state(|| 24.0f64);
    let prediction_confidence = use_state(|| 0.75f64);
    
    // Betting line data
    let betting_spread = use_state(|| -3.5f64);
    let betting_total = use_state(|| 51.5f64);
    let betting_provider = use_state(|| "DraftKings".to_string());
    
    let csv_loading = use_state(|| false);
    let csv_error = use_state(|| None::<String>);
    
    let on_csv_load = {
        let on_bulk_submit = props.on_bulk_submit.clone();
        let csv_loading = csv_loading.clone();
        let csv_error = csv_error.clone();
        
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Some(file_list) = input.files() {
                if let Some(file) = file_list.get(0) {
                    let on_bulk_submit = on_bulk_submit.clone();
                    let csv_loading = csv_loading.clone();
                    let csv_error = csv_error.clone();
                    
                    csv_loading.set(true);
                    csv_error.set(None);
                    
                    if let Ok(reader) = FileReader::new() {
                        let onload = {
                            let reader = reader.clone();
                            let on_bulk_submit = on_bulk_submit.clone();
                            let csv_loading = csv_loading.clone();
                            let csv_error = csv_error.clone();
                            
                            Closure::wrap(Box::new(move |_: web_sys::Event| {
                                if let Ok(result) = reader.result() {
                                    if let Some(content) = result.as_string() {
                                        match parse_csv_data(&content) {
                                            Ok(csv_games) => {
                                                let games: Vec<GameWithPredictionAndLines> = csv_games
                                                    .into_iter()
                                                    .map(csv_to_game_data)
                                                    .collect();
                                                on_bulk_submit.emit(games);
                                            },
                                            Err(e) => {
                                                csv_error.set(Some(format!("CSV parsing error: {}", e)));
                                            }
                                        }
                                    }
                                }
                                csv_loading.set(false);
                            }) as Box<dyn FnMut(_)>)
                        };
                        
                        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                        onload.forget();
                        
                        let _ = reader.read_as_text(&file);
                    } else {
                        csv_error.set(Some("Failed to create file reader".to_string()));
                        csv_loading.set(false);
                    }
                }
            }
        })
    };
    
    let on_submit = {
        let on_submit_callback = props.on_submit.clone();
        let home_team_name = home_team_name.clone();
        let home_team_abbr = home_team_abbr.clone();
        let away_team_name = away_team_name.clone();
        let away_team_abbr = away_team_abbr.clone();
        let home_wins = home_wins.clone();
        let home_losses = home_losses.clone();
        let away_wins = away_wins.clone();
        let away_losses = away_losses.clone();
        let game_week = game_week.clone();
        let game_season = game_season.clone();
        let predicted_home_score = predicted_home_score.clone();
        let predicted_away_score = predicted_away_score.clone();
        let prediction_confidence = prediction_confidence.clone();
        let betting_spread = betting_spread.clone();
        let betting_total = betting_total.clone();
        let betting_provider = betting_provider.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // Create mock game data
            let game_data = create_mock_game_data(
                (*home_team_name).clone(),
                (*home_team_abbr).clone(),
                (*away_team_name).clone(),
                (*away_team_abbr).clone(),
                *home_wins,
                *home_losses,
                *away_wins,
                *away_losses,
                *game_week,
                *game_season,
                *predicted_home_score,
                *predicted_away_score,
                *prediction_confidence,
                *betting_spread,
                *betting_total,
                (*betting_provider).clone(),
            );
            
            on_submit_callback.emit(game_data);
        })
    };

    html! {
        <div class="mock-data-form">
            <h3>{"Add Mock Game Data"}</h3>
            
            <div class="csv-import-section">
                <h4>{"Import from CSV"}</h4>
                <div class="csv-import-controls">
                    <input 
                        type="file" 
                        accept=".csv"
                        onchange={on_csv_load}
                        disabled={*csv_loading}
                    />
                    {if *csv_loading {
                        html! { <span class="loading">{"Loading CSV..."}</span> }
                    } else {
                        html! {}
                    }}
                    {if let Some(error) = (*csv_error).as_ref() {
                        html! { <div class="error">{error}</div> }
                    } else {
                        html! {}
                    }}
                </div>
                <p class="csv-help">{"Select the nfl_predictions.csv file to load all games at once"}</p>
            </div>
            
            <div class="csv-import-section">
                <h4>{"Load NFL Week 3 Data"}</h4>
                <div class="csv-import-controls">
                    <button 
                        type="button"
                        class="submit-btn"
                        onclick={
                            let on_bulk_submit = props.on_bulk_submit.clone();
                            Callback::from(move |_| {
                                let nfl_games = create_nfl_prediction_games();
                                on_bulk_submit.emit(nfl_games);
                            })
                        }
                    >
                        {"Load NFL Week 3 Games with Value Opportunities"}
                    </button>
                </div>
                <p class="csv-help">{"Click to load the actual NFL Week 3 games with probability-based value opportunities"}</p>
            </div>
            
            <div class="divider">{"OR"}</div>
            
            <h4>{"Add Individual Game"}</h4>
            <form onsubmit={on_submit}>
                <div class="form-section">
                    <h4>{"Teams"}</h4>
                    <div class="form-row">
                        <div class="form-group">
                            <label>{"Home Team Name:"}</label>
                            <input 
                                type="text" 
                                value={(*home_team_name).clone()}
                                oninput={
                                    let home_team_name = home_team_name.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        home_team_name.set(input.value());
                                    })
                                }
                            />
                        </div>
                        <div class="form-group">
                            <label>{"Home Team Abbr:"}</label>
                            <input 
                                type="text" 
                                value={(*home_team_abbr).clone()}
                                maxlength="3"
                                oninput={
                                    let home_team_abbr = home_team_abbr.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        home_team_abbr.set(input.value());
                                    })
                                }
                            />
                        </div>
                        <div class="form-group">
                            <label>{"Home Wins:"}</label>
                            <input 
                                type="number" 
                                min="0" 
                                max="17"
                                value={home_wins.to_string()}
                                oninput={
                                    let home_wins = home_wins.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(value) = input.value().parse::<u8>() {
                                            home_wins.set(value);
                                        }
                                    })
                                }
                            />
                        </div>
                        <div class="form-group">
                            <label>{"Home Losses:"}</label>
                            <input 
                                type="number" 
                                min="0" 
                                max="17"
                                value={home_losses.to_string()}
                                oninput={
                                    let home_losses = home_losses.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(value) = input.value().parse::<u8>() {
                                            home_losses.set(value);
                                        }
                                    })
                                }
                            />
                        </div>
                    </div>
                    
                    <div class="form-row">
                        <div class="form-group">
                            <label>{"Away Team Name:"}</label>
                            <input 
                                type="text" 
                                value={(*away_team_name).clone()}
                                oninput={
                                    let away_team_name = away_team_name.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        away_team_name.set(input.value());
                                    })
                                }
                            />
                        </div>
                        <div class="form-group">
                            <label>{"Away Team Abbr:"}</label>
                            <input 
                                type="text" 
                                value={(*away_team_abbr).clone()}
                                maxlength="3"
                                oninput={
                                    let away_team_abbr = away_team_abbr.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        away_team_abbr.set(input.value());
                                    })
                                }
                            />
                        </div>
                        <div class="form-group">
                            <label>{"Away Wins:"}</label>
                            <input 
                                type="number" 
                                min="0" 
                                max="17"
                                value={away_wins.to_string()}
                                oninput={
                                    let away_wins = away_wins.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(value) = input.value().parse::<u8>() {
                                            away_wins.set(value);
                                        }
                                    })
                                }
                            />
                        </div>
                        <div class="form-group">
                            <label>{"Away Losses:"}</label>
                            <input 
                                type="number" 
                                min="0" 
                                max="17"
                                value={away_losses.to_string()}
                                oninput={
                                    let away_losses = away_losses.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(value) = input.value().parse::<u8>() {
                                            away_losses.set(value);
                                        }
                                    })
                                }
                            />
                        </div>
                    </div>
                </div>
                
                <div class="form-section">
                    <h4>{"Game Info"}</h4>
                    <div class="form-row">
                        <div class="form-group">
                            <label>{"Week:"}</label>
                            <input 
                                type="number" 
                                min="1" 
                                max="18"
                                value={game_week.to_string()}
                                oninput={
                                    let game_week = game_week.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(value) = input.value().parse::<u8>() {
                                            game_week.set(value);
                                        }
                                    })
                                }
                            />
                        </div>
                        <div class="form-group">
                            <label>{"Season:"}</label>
                            <input 
                                type="number" 
                                min="2020" 
                                max="2030"
                                value={game_season.to_string()}
                                oninput={
                                    let game_season = game_season.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(value) = input.value().parse::<u16>() {
                                            game_season.set(value);
                                        }
                                    })
                                }
                            />
                        </div>
                    </div>
                </div>
                
                <div class="form-section">
                    <h4>{"Prediction Data"}</h4>
                    <div class="form-row">
                        <div class="form-group">
                            <label>{"Predicted Home Score:"}</label>
                            <input 
                                type="number" 
                                step="0.1"
                                min="0"
                                max="70"
                                value={predicted_home_score.to_string()}
                                oninput={
                                    let predicted_home_score = predicted_home_score.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(value) = input.value().parse::<f64>() {
                                            predicted_home_score.set(value);
                                        }
                                    })
                                }
                            />
                        </div>
                        <div class="form-group">
                            <label>{"Predicted Away Score:"}</label>
                            <input 
                                type="number" 
                                step="0.1"
                                min="0"
                                max="70"
                                value={predicted_away_score.to_string()}
                                oninput={
                                    let predicted_away_score = predicted_away_score.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(value) = input.value().parse::<f64>() {
                                            predicted_away_score.set(value);
                                        }
                                    })
                                }
                            />
                        </div>
                        <div class="form-group">
                            <label>{"Confidence (0-1):"}</label>
                            <input 
                                type="number" 
                                step="0.01"
                                min="0"
                                max="1"
                                value={prediction_confidence.to_string()}
                                oninput={
                                    let prediction_confidence = prediction_confidence.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(value) = input.value().parse::<f64>() {
                                            prediction_confidence.set(value);
                                        }
                                    })
                                }
                            />
                        </div>
                    </div>
                </div>
                
                <div class="form-section">
                    <h4>{"Betting Line"}</h4>
                    <div class="form-row">
                        <div class="form-group">
                            <label>{"Spread (Home):"}</label>
                            <input 
                                type="number" 
                                step="0.5"
                                min="-21"
                                max="21"
                                value={betting_spread.to_string()}
                                oninput={
                                    let betting_spread = betting_spread.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(value) = input.value().parse::<f64>() {
                                            betting_spread.set(value);
                                        }
                                    })
                                }
                            />
                        </div>
                        <div class="form-group">
                            <label>{"Total:"}</label>
                            <input 
                                type="number" 
                                step="0.5"
                                min="30"
                                max="70"
                                value={betting_total.to_string()}
                                oninput={
                                    let betting_total = betting_total.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(value) = input.value().parse::<f64>() {
                                            betting_total.set(value);
                                        }
                                    })
                                }
                            />
                        </div>
                        <div class="form-group">
                            <label>{"Provider:"}</label>
                            <select 
                                value={(*betting_provider).clone()}
                                onchange={
                                    let betting_provider = betting_provider.clone();
                                    Callback::from(move |e: Event| {
                                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                        betting_provider.set(select.value());
                                    })
                                }
                            >
                                <option value="DraftKings">{"DraftKings"}</option>
                                <option value="FanDuel">{"FanDuel"}</option>
                                <option value="BetMGM">{"BetMGM"}</option>
                                <option value="Caesars">{"Caesars"}</option>
                            </select>
                        </div>
                    </div>
                </div>
                
                <div class="form-actions">
                    <button type="submit" class="submit-btn">{"Add Game"}</button>
                </div>
            </form>
        </div>
    }
}

fn create_mock_game_data(
    home_team_name: String,
    home_team_abbr: String,
    away_team_name: String,
    away_team_abbr: String,
    home_wins: u8,
    home_losses: u8,
    away_wins: u8,
    away_losses: u8,
    game_week: u8,
    game_season: u16,
    predicted_home_score: f64,
    predicted_away_score: f64,
    prediction_confidence: f64,
    betting_spread: f64,
    betting_total: f64,
    betting_provider: String,
) -> GameWithPredictionAndLines {
    // Create teams with stats
    let mut home_team = Team::new(home_team_name, home_team_abbr);
    home_team.stats.wins = home_wins;
    home_team.stats.losses = home_losses;
    home_team.stats.games_played = home_wins + home_losses;
    home_team.stats.offensive_rating = 85.0 + (home_wins as f64 - 8.0) * 2.0;
    home_team.stats.defensive_rating = 80.0 + (home_wins as f64 - 8.0) * 1.5;
    
    let mut away_team = Team::new(away_team_name, away_team_abbr);
    away_team.stats.wins = away_wins;
    away_team.stats.losses = away_losses;
    away_team.stats.games_played = away_wins + away_losses;
    away_team.stats.offensive_rating = 85.0 + (away_wins as f64 - 8.0) * 2.0;
    away_team.stats.defensive_rating = 80.0 + (away_wins as f64 - 8.0) * 1.5;
    
    // Create game (scheduled for next Sunday at 1 PM)
    let game_time = Utc::now() + chrono::Duration::days(7);
    let game = Game::new(home_team, away_team, game_time, game_week, game_season);
    
    // Create prediction
    let home_samples: Vec<f64> = (0..100).map(|i| {
        predicted_home_score + (i as f64 - 50.0) * 0.2
    }).collect();
    
    let away_samples: Vec<f64> = (0..100).map(|i| {
        predicted_away_score + (i as f64 - 50.0) * 0.2
    }).collect();
    
    let home_distribution = ProbabilityDistribution::new(home_samples);
    let away_distribution = ProbabilityDistribution::new(away_samples);
    
    let prediction = GamePrediction::new(
        game.id.clone(),
        home_distribution,
        away_distribution,
    );
    
    // Create betting line
    let betting_line = BettingLine::new(
        game.id.clone(),
        betting_provider,
        betting_spread,
        betting_total,
        -110,
        -110,
    );
    
    // Create value opportunities if there's a significant difference
    let mut value_opportunities = Vec::new();
    let spread_diff = (prediction.spread_prediction - betting_spread).abs();
    let total_diff = (prediction.total_prediction - betting_total).abs();
    
    // Only show value opportunities if there's significant difference AND reasonable confidence
    if spread_diff > 2.0 && prediction_confidence > 0.2 {
        // Simple logic: determine which team is undervalued by the market
        // If model prediction > market spread, then away team is undervalued (bet away team)
        // If model prediction < market spread, then home team is undervalued (bet home team)
        
        let recommendation = if prediction.spread_prediction < betting_spread {
            // Model thinks away team should be MORE favored than market
            // (more negative spread_prediction than betting_spread)
            // Value is on the away team
            format!("{}: -{:.1}", game.away_team.abbreviation, betting_spread.abs())
        } else {
            // Model thinks home team should be MORE favored than market
            // (more positive spread_prediction than betting_spread)  
            // Value is on the home team
            format!("{}: -{:.1}", game.home_team.abbreviation, betting_spread.abs())
        };
        
        let opportunity = ValueOpportunity::new(
            game.id.clone(),
            OpportunityType::SpreadValue,
            prediction_confidence,
            spread_diff,
            recommendation,
            betting_line.id.clone(),
        );
        value_opportunities.push(opportunity);
    }
    
    if total_diff > 3.0 {
        let opportunity = ValueOpportunity::new(
            game.id.clone(),
            OpportunityType::TotalValue,
            prediction_confidence,
            total_diff,
            if prediction.total_prediction > betting_total {
                format!("Consider betting OVER {:.1}", betting_total)
            } else {
                format!("Consider betting UNDER {:.1}", betting_total)
            },
            betting_line.id.clone(),
        );
        value_opportunities.push(opportunity);
    }
    
    GameWithPredictionAndLines {
        game,
        prediction: Some(prediction),
        betting_lines: vec![betting_line],
        value_opportunities,
    }
}