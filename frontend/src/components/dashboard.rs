use yew::prelude::*;
use share::models::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::game_card::GameCard;
use super::mock_data_form::MockDataForm;

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
    let show_mock_form = use_state(|| false);
    
    let toggle_mock_form = {
        let show_mock_form = show_mock_form.clone();
        Callback::from(move |_| {
            show_mock_form.set(!*show_mock_form);
        })
    };

    let on_mock_data_submit = {
        let on_game_update = props.on_game_update.clone();
        let show_mock_form = show_mock_form.clone();
        Callback::from(move |game_data: GameWithPredictionAndLines| {
            on_game_update.emit(game_data);
            show_mock_form.set(false);
        })
    };

    let on_bulk_mock_data_submit = {
        let on_bulk_game_update = props.on_bulk_game_update.clone();
        let show_mock_form = show_mock_form.clone();
        Callback::from(move |games: Vec<GameWithPredictionAndLines>| {
            on_bulk_game_update.emit(games);
            show_mock_form.set(false);
        })
    };

    html! {
        <div class="dashboard">
            <header class="dashboard-header">
                <h1>{"NFL Prediction Dashboard"}</h1>
                <button 
                    class="mock-data-btn"
                    onclick={toggle_mock_form}
                >
                    {if *show_mock_form { "Hide Mock Data Form" } else { "Add Mock Game" }}
                </button>
            </header>

            {if *show_mock_form {
                html! {
                    <div class="mock-form-container">
                        <MockDataForm 
                            on_submit={on_mock_data_submit}
                            on_bulk_submit={on_bulk_mock_data_submit}
                        />
                    </div>
                }
            } else {
                html! {}
            }}

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