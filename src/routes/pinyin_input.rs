use askama_axum::{IntoResponse, Template};

#[derive(Template)]
#[template(
    path = "views/pinyin_input.html",
    config = "assets/templates/config.toml",
    escape = "none"
)]
struct PinyinInputTemplate;

pub async fn route() -> impl IntoResponse {
    PinyinInputTemplate
}
