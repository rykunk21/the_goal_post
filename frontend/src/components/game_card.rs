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
    
    // Format game time (unused in simplified UI)
    let _game_time_str = game.game_time.format("%m/%d %I:%M %p").to_string();
    
    // Get primary betting line (first one if available)
    let primary_line = game_data.betting_lines.first();
    
    // Check for value opportunities
    let has_value = !game_data.value_opportunities.is_empty();
    let value_class = if has_value { "has-value" } else { "" };

    html! {
        <div class={classes!("game-card", value_class)}>
            <div class="matchup-container">
                <div class="team-info away-team">
                    <div class="team-abbr">{&game.away_team.abbreviation}</div>
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
                            {if let Some(community_pos) = prediction_marker {
                                html! {
                                    <div 
                                        class="prediction-marker" 
                                        style={format!("left: {}%", community_pos)}
                                        title="Community Prediction"
                                    >
                                        <div class="marker-label">{"C"}</div>
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                            
                            {if let Some(market_pos) = book_marker {
                                html! {
                                    <div 
                                        class="book-marker" 
                                        style={format!("left: {}%", market_pos)}
                                        title="Market Odds"
                                    >
                                        <div class="marker-label">{"M"}</div>
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
                    <div class="team-abbr">{&game.home_team.abbreviation}</div>
                </div>
            </div>
            
            {if has_value {
                html! {
                    <div class="value-opportunities">
                        {for game_data.value_opportunities.iter().map(|opportunity| {
                            let (bet_line, value_percentage) = format_betting_recommendation(
                                opportunity, 
                                game, 
                                primary_line
                            );
                            
                            // Calculate confidence score based on value differential
                            let confidence_score = calculate_confidence_score(value_percentage);
                            
                            html! {
                                <div class="value-item">
                                    <div class="bet-recommendation">
                                        {bet_line}
                                    </div>
                                    <div class="value-info">
                                        <div class="value-percentage">
                                            {format!("{:+.1}%", value_percentage)}
                                        </div>
                                        <div class="confidence-score">
                                            {format!("Confidence: {}", confidence_score)}
                                        </div>
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

fn format_betting_recommendation(
    opportunity: &ValueOpportunity, 
    game: &Game, 
    primary_line: Option<&BettingLine>
) -> (String, f64) {
    // Extract value percentage from expected_value (assuming it's stored as a decimal)
    let value_percentage = opportunity.expected_value * 100.0;
    
    // Parse the recommendation to extract team and spread
    // Expected format: "CAR +5.5" or "ATL -3.5" etc.
    if let Some(line) = primary_line {
        match opportunity.opportunity_type {
            OpportunityType::SpreadValue => {
                // For spread value, format as "TEAM +/-X.X"
                let bet_line = if opportunity.recommendation.contains(&game.home_team.abbreviation) {
                    // Home team value
                    if line.spread > 0.0 {
                        format!("{} +{:.1}", game.home_team.abbreviation, line.spread)
                    } else if line.spread < 0.0 {
                        format!("{} {:.1}", game.home_team.abbreviation, line.spread)
                    } else {
                        format!("{} EVEN", game.home_team.abbreviation)
                    }
                } else {
                    // Away team value (opposite of home spread)
                    let away_spread = -line.spread;
                    if away_spread > 0.0 {
                        format!("{} +{:.1}", game.away_team.abbreviation, away_spread)
                    } else if away_spread < 0.0 {
                        format!("{} {:.1}", game.away_team.abbreviation, away_spread)
                    } else {
                        format!("{} EVEN", game.away_team.abbreviation)
                    }
                };
                (bet_line, value_percentage)
            },
            OpportunityType::TotalValue => {
                // For total value, format as "OVER/UNDER X.X"
                if opportunity.recommendation.to_lowercase().contains("over") {
                    (format!("OVER {:.1}", line.total), value_percentage)
                } else {
                    (format!("UNDER {:.1}", line.total), value_percentage)
                }
            },
            _ => {
                // Fallback to original recommendation
                (opportunity.recommendation.clone(), value_percentage)
            }
        }
    } else {
        // No betting line available, use original recommendation
        (opportunity.recommendation.clone(), value_percentage)
    }
}

fn calculate_matchup_visualization(game_data: &GameWithPredictionAndLines) -> (f64, f64, Option<f64>, Option<f64>) {
    // Calculate probability-based visualization
    // This should reflect the community vs market probability differential
    
    if let Some(line) = game_data.betting_lines.first() {
        // Convert spread to implied probabilities using logistic model
        let market_home_prob = spread_to_probability(-line.spread) * 100.0; // Convert to percentage
        let market_away_prob = 100.0 - market_home_prob;
        
        // For community probabilities, we'll derive them from the value opportunities
        // If there's a value opportunity, it means community differs from market
        let (community_home_prob, community_away_prob) = if let Some(value_opp) = game_data.value_opportunities.first() {
            // Extract the probability differential from the expected value
            let value_diff = value_opp.expected_value * 100.0; // Convert to percentage
            
            if value_opp.recommendation.contains(&game_data.game.home_team.abbreviation) {
                // Value is on home team, so community thinks home team has higher probability
                let community_home = (market_home_prob + value_diff.abs()).min(95.0).max(5.0);
                (community_home, 100.0 - community_home)
            } else {
                // Value is on away team, so community thinks away team has higher probability  
                let community_away = (market_away_prob + value_diff.abs()).min(95.0).max(5.0);
                (100.0 - community_away, community_away)
            }
        } else {
            // No value opportunity, so community and market agree
            (market_home_prob, market_away_prob)
        };
        
        // Use community probabilities for the gradient (this shows what the community thinks)
        let home_strength = community_home_prob;
        let away_strength = community_away_prob;
        
        // Market position (where the betting line thinks the game should be)
        let market_position = market_home_prob;
        
        // Community position (where the community thinks the game should be)  
        let community_position = community_home_prob;
        
        (home_strength, away_strength, Some(community_position), Some(market_position))
    } else {
        // No betting line available, use neutral
        (50.0, 50.0, None, None)
    }
}

// Helper function to convert spread to probability (same as in our Python analysis)
fn spread_to_probability(spread: f64) -> f64 {
    if spread == 0.0 {
        return 0.5;
    }
    // Using logistic function: P = 1 / (1 + e^(-spread/3.3))
    1.0 / (1.0 + (-spread / 3.3).exp())
}

// Calculate confidence score based on value differential
fn calculate_confidence_score(value_percentage: f64) -> String {
    let abs_value = value_percentage.abs();
    
    if abs_value >= 15.0 {
        "★★★★★".to_string() // 5 stars for 15%+ value
    } else if abs_value >= 12.0 {
        "★★★★☆".to_string() // 4 stars for 12-15% value
    } else if abs_value >= 9.0 {
        "★★★☆☆".to_string() // 3 stars for 9-12% value
    } else if abs_value >= 6.0 {
        "★★☆☆☆".to_string() // 2 stars for 6-9% value
    } else if abs_value >= 3.0 {
        "★☆☆☆☆".to_string() // 1 star for 3-6% value
    } else {
        "☆☆☆☆☆".to_string() // No stars for <3% value
    }
}