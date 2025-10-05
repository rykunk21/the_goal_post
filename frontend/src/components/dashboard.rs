use yew::prelude::*;
use share::models::*;
use chrono::{DateTime, Utc, Datelike};
use std::collections::HashMap;

use super::game_card::GameCard;

#[derive(Properties, PartialEq)]
pub struct DashboardProps {
    pub games: Vec<GameWithPredictionAndLines>,
    pub on_game_update: Callback<GameWithPredictionAndLines>,
    pub on_bulk_game_update: Callback<Vec<GameWithPredictionAndLines>>,
}

#[derive(Clone, PartialEq)]
pub struct GameWithPredictionAndLines {
    pub game: Game,
    pub prediction: Option<GamePrediction>,
    pub betting_lines: Vec<BettingLine>,
    pub value_opportunities: Vec<ValueOpportunity>,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {


    // Auto-load current week data on component mount
    let current_week = get_current_nfl_week();
    let games_loaded = use_state(|| false);
    
    {
        let on_bulk_game_update = props.on_bulk_game_update.clone();
        let games_loaded = games_loaded.clone();
        use_effect_with((), move |_| {
            if !*games_loaded {
                let nfl_games = load_nfl_week_data(current_week);
                on_bulk_game_update.emit(nfl_games);
                games_loaded.set(true);
            }
            || ()
        });
    }

    html! {
        <div class="dashboard">
            <header class="dashboard-header">
                <h1>{format!("NFL Week {} Predictions", current_week)}</h1>
                <div class="week-info">
                    <span class="current-week">{"Current Week: "}{current_week}</span>
                </div>
            </header>

            <main class="dashboard-content">
                {if props.games.is_empty() {
                    html! {
                        <div class="empty-state">
                            <h2>{"No games available"}</h2>
                            <p>{"Add some mock game data to get started"}</p>
                        </div>
                    }
                } else {
                    html! {
                        <div class="games-grid">
                            {for props.games.iter().map(|game_data| {
                                html! {
                                    <GameCard
                                        key={game_data.game.id.clone()}
                                        game_data={game_data.clone()}
                                    />
                                }
                            })}
                        </div>
                    }
                }}
            </main>
        </div>
    }
}

// Get the current NFL week based on the date
fn get_current_nfl_week() -> u8 {
    // For now, hardcode to Week 3 since that's our current dataset
    // TODO: Implement proper date-based week calculation when we have more weeks of data
    3
    
    /* Future implementation:
    let now = Utc::now();
    
    // NFL 2024 season started September 5, 2024 (Week 1)
    let season_start = chrono::NaiveDate::from_ymd_opt(2024, 9, 5)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let season_start_utc = DateTime::from_naive_utc_and_offset(season_start, Utc);
    
    if now < season_start_utc {
        return 1;
    }
    
    let days_since_start = (now - season_start_utc).num_days();
    let week = (days_since_start / 7) + 1;
    
    std::cmp::min(week as u8, 18)
    */
}

// Load NFL data for a specific week
fn load_nfl_week_data(week: u8) -> Vec<GameWithPredictionAndLines> {
    match week {
        3 => load_week_3_data(),
        _ => {
            // For other weeks, we could load from different sources
            // For now, default to week 3 data as a fallback
            web_sys::console::log_1(&format!("Week {} data not available, using Week 3 as fallback", week).into());
            load_week_3_data()
        }
    }
}

// Load Week 3 data (our current dataset)
fn load_week_3_data() -> Vec<GameWithPredictionAndLines> {
    // Updated data based on latest probability analysis from temp/probability_analysis.py
    // Format: (away_name, away_abbr, home_name, home_abbr, away_score, home_score, confidence, spread, total, bet_rec, value_pct)
    let games_data: Vec<(&str, &str, &str, &str, f64, f64, f64, f64, f64, &str, f64)> = vec![
        ("Atlanta Falcons", "ATL", "Carolina Panthers", "CAR", 26.4, 21.6, 0.34, 4.5, 45.0, "CAR +4.5", 12.6),
        ("Green Bay Packers", "GB", "Cleveland Browns", "CLE", 28.3, 19.7, 0.62, 8.5, 45.0, "CLE +8.5", 11.9),
        ("Houston Texans", "HOU", "Jacksonville Jaguars", "JAX", 23.6, 24.4, 0.06, -1.5, 45.0, "JAX -1.5", -8.2),
        ("Cincinnati Bengals", "CIN", "Minnesota Vikings", "MIN", 22.6, 25.4, 0.2, -3.5, 45.0, "MIN -3.5", -14.3),
        ("Pittsburgh Steelers", "PIT", "New England Patriots", "NE", 24.7, 23.3, 0.1, 1.5, 45.0, "NE +1.5", 6.2),
        ("Los Angeles Rams", "LA", "Philadelphia Eagles", "PHI", 22.2, 25.8, 0.26, -3.5, 45.0, "PHI -3.5", -11.3),
        ("New York Jets", "NYJ", "Tampa Bay Buccaneers", "TB", 20.5, 27.5, 0.5, -6.5, 45.0, "TB -6.5", -12.8),
        ("Indianapolis Colts", "IND", "Tennessee Titans", "TEN", 26.1, 21.9, 0.3, 5.5, 45.0, "TEN +5.5", 19.1),
        ("Las Vegas Raiders", "LV", "Washington Commanders", "WAS", 22.7, 25.3, 0.18, -3.0, 45.0, "LV +3.0", 12.3),
        ("Denver Broncos", "DEN", "Los Angeles Chargers", "LAC", 22.7, 25.3, 0.18, -3.0, 45.0, "DEN +3.0", 12.3),
        ("New Orleans Saints", "NO", "Seattle Seahawks", "SEA", 20.1, 27.9, 0.56, -7.5, 45.0, "NO +7.5", 12.7),
        ("Dallas Cowboys", "DAL", "Chicago Bears", "CHI", 24.3, 23.7, 0.04, 1.5, 45.0, "CHI +1.5", 9.2),
        ("Arizona Cardinals", "ARI", "San Francisco 49ers", "SF", 23.3, 24.7, 0.1, 0.0, 45.0, "SF EVEN", 5.0),
        ("Kansas City Chiefs", "KC", "New York Giants", "NYG", 27.4, 20.6, 0.48, 6.5, 45.0, "NYG +6.5", 13.8),
        ("Detroit Lions", "DET", "Baltimore Ravens", "BAL", 21.5, 26.5, 0.36, -4.5, 45.0, "BAL -4.5", -11.6),
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
                    percentiles: HashMap::new(),
                },
                away_score_distribution: ProbabilityDistribution {
                    mean: away_score,
                    std_dev: 7.0,
                    samples: vec![away_score - 3.0, away_score, away_score + 3.0],
                    percentiles: HashMap::new(),
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