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
    let click_callback = Callback::from(|_| {});
    html! {
        <>
        <header>
            <h1>{ "工事中" }</h1>
            <nav>
            <ul class="nav justify-content-end">
            <li class="nav-item">
            <a class="nav-link disabled" href="#" tabindex="-1" aria-disabled="true">{"問題追加"}</a>
            </li>
            <li class="nav-item">
            <a class="nav-link active" aria-current="page" href="#">{"サインイン"}</a>
            </li>
            <li class="nav-item">
            <a class="nav-link" href="#">{"サインアップ"}</a>
            </li>
        </ul>
        </nav>
        </header>
        <button onclick={click_callback}>
                { "押しても何も起きません" }
        </button>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
