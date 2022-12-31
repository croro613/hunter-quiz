use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct QuizPageViewProps {}

#[function_component(QuizPageView)]
pub fn quiz_page_view(props: &QuizPageViewProps) -> Html {
    html! {
        { "Hello world" }
    }
}
