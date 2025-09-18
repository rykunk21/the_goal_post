use yew::prelude::*;
use share::*;

mod components;

use components::{Dashboard, GameWithPredictionAndLines};

#[function_component(App)]
fn app() -> Html {
    let games = use_state(|| Vec::<GameWithPredictionAndLines>::new());
    
    let on_game_update = {
        let games = games.clone();
        Callback::from(move |new_game: GameWithPredictionAndLines| {
            let mut updated_games = (*games).clone();
            updated_games.push(new_game);
            games.set(updated_games);
        })
    };

    html! {
        <div class="app">
            <Dashboard 
                games={(*games).clone()}
                on_game_update={on_game_update}
            />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
