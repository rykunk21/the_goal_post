use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1> {"Hello to the new app!"} </h1>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
