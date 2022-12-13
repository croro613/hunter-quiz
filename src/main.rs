use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/add_quiz")]
    AddQuiz,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    let click_callback = Callback::from(|_| {
    
    });
    html! {
        <>
        <h1>{ "Hunterクイズ工事中" }</h1>
        <button onclick={click_callback}>
                { "押しても何も起きません" }
        </button>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}