use postgrest::Postgrest;
use yew::use_effect_with_deps;
use yew::{function_component, html, use_effect, use_state, Html, Properties};

const SUPABASE_API_KEY:&str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Imh1amhpc250c2dkcm52a3lla2ptIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzA3NDY3MzYsImV4cCI6MTk4NjMyMjczNn0.sEYx4g18lAQsf3X7F2X7AYeEZzoKs_o9yAkasbCgdsA";

#[derive(Properties, PartialEq)]
pub struct QuizPageViewProps {}

#[function_component(QuizPageView)]
pub fn quiz_page_view(props: &QuizPageViewProps) -> Html {
    let quiz_details = use_state(|| String::from("no_data"));
    {
        let quiz_details = quiz_details.clone();
        use_effect_with_deps(
            move |_| {
                let quiz_details = quiz_details.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client =
                        Postgrest::new("https://hujhisntsgdrnvkyekjm.supabase.co/rest/v1/")
                            .insert_header("apikey", SUPABASE_API_KEY);
                    let res = client
                        .from("quiz_info")
                        .select("*")
                        .execute()
                        .await
                        .unwrap();
                    log::info!("{:?}", res);
                    let resp = res.text().await.unwrap();
                    quiz_details.set(resp);
                });
                || ()
            },
            (),
        );
    }
    html! {
        {(*quiz_details).clone()}
    }
}

async fn fetch_postgrest() -> String {
    log::info!("呼ばれたよ");
    let client = Postgrest::new("https://hujhisntsgdrnvkyekjm.supabase.co/rest/v1/")
        .insert_header("apikey", SUPABASE_API_KEY);
    let res = client.from("quiz_info").select("*").execute().await;
    if let Ok(r) = res {
        log::info!("レスポンス成功");
        let tmp = r.text().await;
        if let Ok(t) = tmp {
            return t;
        } else {
            return String::from("");
        }
    } else {
        log::info!("{:?}", res);
        return String::from("");
    }

    // match fetched_forecast {
    //     Ok(response) => {
    //         let json: Result<Forecast, _> = response.json().await;
    //         match json {
    //             Ok(f) => {
    //                 forecast.set(Some(f));
    //             }
    //             Err(e) => error.set(Some(e.to_string())),
    //         }
    //     }
    //     Err(e) => error.set(Some(e.to_string())),
    // }
}
