use quiz_page::quiz_page_view::QuizPageView;
use yew::prelude::*;
use yew_router::prelude::*;

mod quiz_page;

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    // #[at("/add_quiz")]
    // AddQuiz,
    #[at("/quiz_page")]
    QuizPageView,
    // #[not_found]
    // #[at("/404")]
    // NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::QuizPageView => html! {
        <QuizPageView/>
        },
    }
}

#[function_component(Home)]
fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::QuizPageView));
    html! {
        <>

      <div class="first-select-button">
      <div class="inner">
      <button type="button" class="btn btn-outline-primary inner-button" onclick={onclick}>{ "ひとりで遊ぶ" }</button>
      <button type="button" class="btn btn-outline-warning inner-button" >{ "みんなで遊ぶ" }</button>
      </div>
      </div>
      </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
        <nav class="navbar sticky-top navbar-light bg-light">
        <div class="container-fluid header-contents">
          <Link<Route> classes={classes!("navbar-brand")} to={Route::Home}>{ "Hunter Quiz" }</Link<Route>>
        </div>
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
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    println!("Hello");
    yew::Renderer::<App>::new().render();
}
