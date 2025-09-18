use yew::prelude::*;
use web_sys::HtmlInputElement;
use share::models::*;
use chrono::{DateTime, Utc, TimeZone};
use std::collections::HashMap;

use super::dashboard::GameWithPredictionAndLines;

#[derive(Properties, PartialEq)]
pub struct MockDataFormProps {
    pub on_submit: Callback<GameWithPredictionAndLines>,
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
    
    if spread_diff > 2.0 {
        let opportunity = ValueOpportunity::new(
            game.id.clone(),
            OpportunityType::SpreadValue,
            prediction_confidence,
            spread_diff,
            if prediction.spread_prediction > betting_spread {
                "Consider betting the underdog".to_string()
            } else {
                "Consider betting the favorite".to_string()
            },
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