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
        <nav class="navbar sticky-top navbar-light bg-light">
        <div class="container-fluid header-contents">
          <a class="navbar-brand display-block" href="#">{"Hunter✖️クイズ"}</a>
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
      <div class="first-select-button">
      <div class="inner">
      <button type="button" class="btn btn-outline-primary inner-button" onclick={click_callback}>{ "ひとりで遊ぶ" }</button>
      <button type="button" class="btn btn-outline-warning inner-button">{ "みんなで遊ぶ" }</button>
      </div>
      </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
