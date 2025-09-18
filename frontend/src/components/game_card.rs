use yew::prelude::*;
use share::models::*;
use chrono::{DateTime, Utc};

use super::dashboard::GameWithPredictionAndLines;

#[derive(Properties, PartialEq)]
pub struct GameCardProps {
    pub game_data: GameWithPredictionAndLines,
}

#[function_component(GameCard)]
pub fn game_card(props: &GameCardProps) -> Html {
    let game_data = &props.game_data;
    let game = &game_data.game;
    
    // Calculate gradient position based on prediction and betting lines
    let (home_strength, away_strength, prediction_marker, book_marker) = calculate_matchup_visualization(game_data);
    
    // Format game time
    let game_time_str = game.game_time.format("%m/%d %I:%M %p").to_string();
    
    // Get primary betting line (first one if available)
    let primary_line = game_data.betting_lines.first();
    
    // Check for value opportunities
    let has_value = !game_data.value_opportunities.is_empty();
    let value_class = if has_value { "has-value" } else { "" };

    html! {
        <div class={classes!("game-card", value_class)}>
            <div class="game-header">
                <div class="game-time">{game_time_str}</div>
                <div class="game-week">{format!("Week {}", game.week)}</div>
            </div>
            
            <div class="matchup-container">
                <div class="team-info away-team">
                    <div class="team-name">{&game.away_team.name}</div>
                    <div class="team-abbr">{&game.away_team.abbreviation}</div>
                    <div class="team-record">
                        {format!("{}-{}-{}", 
                            game.away_team.stats.wins, 
                            game.away_team.stats.losses, 
                            game.away_team.stats.ties
                        )}
                    </div>
                </div>
                
                <div class="vs-section">
                    <div class="gradient-bar-container">
                        <div class="gradient-bar" style={format!(
                            "background: linear-gradient(to right, 
                                var(--away-color) 0%, 
                                var(--away-color) {}%, 
                                var(--home-color) {}%, 
                                var(--home-color) 100%)",
                            away_strength, home_strength
                        )}>
                            {if let Some(prediction_pos) = prediction_marker {
                                html! {
                                    <div 
                                        class="prediction-marker" 
                                        style={format!("left: {}%", prediction_pos)}
                                        title="Model Prediction"
                                    >
                                        <div class="marker-label">{"M"}</div>
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                            
                            {if let Some(book_pos) = book_marker {
                                html! {
                                    <div 
                                        class="book-marker" 
                                        style={format!("left: {}%", book_pos)}
                                        title="Betting Line"
                                    >
                                        <div class="marker-label">{"B"}</div>
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                        </div>
                        
                        <div class="gradient-labels">
                            <span class="away-label">{&game.away_team.abbreviation}</span>
                            <span class="home-label">{&game.home_team.abbreviation}</span>
                        </div>
                    </div>
                </div>
                
                <div class="team-info home-team">
                    <div class="team-name">{&game.home_team.name}</div>
                    <div class="team-abbr">{&game.home_team.abbreviation}</div>
                    <div class="team-record">
                        {format!("{}-{}-{}", 
                            game.home_team.stats.wins, 
                            game.home_team.stats.losses, 
                            game.home_team.stats.ties
                        )}
                    </div>
                </div>
            </div>
            
            <div class="prediction-info">
                {if let Some(prediction) = &game_data.prediction {
                    html! {
                        <div class="prediction-details">
                            <div class="prediction-row">
                                <span class="label">{"Predicted Winner:"}</span>
                                <span class="value predicted-winner">
                                    {if prediction.spread_prediction > 0.0 {
                                        format!("{} by {:.1}", game.home_team.abbreviation, prediction.spread_prediction)
                                    } else {
                                        format!("{} by {:.1}", game.away_team.abbreviation, prediction.spread_prediction.abs())
                                    }}
                                </span>
                            </div>
                            <div class="prediction-row">
                                <span class="label">{"Total Points:"}</span>
                                <span class="value">{format!("{:.1}", prediction.total_prediction)}</span>
                            </div>
                            <div class="prediction-row">
                                <span class="label">{"Confidence:"}</span>
                                <span class="value">{format!("{:.1}%", prediction.home_win_probability() * 100.0)}</span>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="no-prediction">
                            <span>{"No prediction available"}</span>
                        </div>
                    }
                }}
            </div>
            
            {if let Some(line) = primary_line {
                html! {
                    <div class="betting-info">
                        <div class="betting-row">
                            <span class="label">{"Spread:"}</span>
                            <span class="value">{format!("{:+.1}", line.spread)}</span>
                        </div>
                        <div class="betting-row">
                            <span class="label">{"Total:"}</span>
                            <span class="value">{format!("{:.1}", line.total)}</span>
                        </div>
                        <div class="betting-row">
                            <span class="label">{"Provider:"}</span>
                            <span class="value">{&line.provider}</span>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
            
            {if has_value {
                html! {
                    <div class="value-opportunities">
                        <div class="value-header">{"Value Opportunities:"}</div>
                        {for game_data.value_opportunities.iter().map(|opportunity| {
                            html! {
                                <div class="value-item">
                                    <span class="opportunity-type">
                                        {format!("{:?}", opportunity.opportunity_type)}
                                    </span>
                                    <span class="confidence">
                                        {format!("{:.0}%", opportunity.confidence * 100.0)}
                                    </span>
                                    <div class="recommendation">
                                        {&opportunity.recommendation}
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}

fn calculate_matchup_visualization(game_data: &GameWithPredictionAndLines) -> (f64, f64, Option<f64>, Option<f64>) {
    let game = &game_data.game;
    
    // Base team strength calculation (simplified)
    let home_rating = game.home_team.stats.offensive_rating + game.home_team.stats.defensive_rating;
    let away_rating = game.away_team.stats.offensive_rating + game.away_team.stats.defensive_rating;
    
    // Normalize to 0-100 scale for gradient
    let total_rating = home_rating + away_rating;
    let away_strength = if total_rating > 0.0 {
        (away_rating / total_rating * 100.0).max(10.0).min(90.0)
    } else {
        50.0
    };
    let home_strength = 100.0 - away_strength;
    
    // Calculate marker positions based on predictions and betting lines
    let prediction_marker = if let Some(prediction) = &game_data.prediction {
        // Convert spread to position on 0-100 scale
        // Negative spread means home team favored, positive means away team favored
        let spread_position = 50.0 - (prediction.spread_prediction * 2.0); // Scale factor of 2
        Some(spread_position.max(5.0).min(95.0))
    } else {
        None
    };
    
    let book_marker = if let Some(line) = game_data.betting_lines.first() {
        // Convert betting line spread to position
        let line_position = 50.0 - (line.spread * 2.0); // Scale factor of 2
        Some(line_position.max(5.0).min(95.0))
    } else {
        None
    };
    
    (home_strength, away_strength, prediction_marker, book_marker)
}