use std::sync::{Arc, Mutex};

use axum::{
    Json, Router,
    extract::State,
    response::Html,
    routing::{get, post},
};
use fuzzy_matcher::skim::SkimMatcherV2;
use serde::Deserialize;

use crate::{
    config::Config,
    item::Item,
    item_source::ItemSource,
    matcher::{new_matcher, update_scores},
};

#[derive(Clone)]
struct AppState {
    pub items: Arc<Mutex<Vec<Item>>>,
    pub query: Arc<Mutex<String>>,
    pub matcher: Arc<SkimMatcherV2>,
}

impl AppState {
    pub fn new(config: Config) -> anyhow::Result<Self> {
        let mut source = ItemSource::new(&config);
        let mut items = source.get_items(&config.query)?;
        let matcher = new_matcher(config.case);
        update_scores(&config.query, &matcher, &mut items);
        Ok(AppState {
            items: Arc::new(Mutex::new(items)),
            query: Arc::new(Mutex::new(config.query.clone())),
            matcher: Arc::new(matcher),
        })
    }
}

pub async fn run(
    config: Config,
) -> anyhow::Result<axum::serve::Serve<tokio::net::TcpListener, Router, Router>> {
    tracing_subscriber::fmt::init();
    let state = AppState::new(config)?;
    let app = Router::new()
        .route("/", get(root))
        .route("/fuzzy-match", post(fuzzy_match))
        .route("/submit", post(submit))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    Ok(axum::serve(listener, app))
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[derive(Deserialize)]
struct MatchRequest {
    query: String,
}

async fn fuzzy_match(
    State(state): State<AppState>,
    Json(req): Json<MatchRequest>,
) -> Json<Vec<Item>> {
    let mut query = state.query.lock().expect("mutex was poisoned");
    *query = req.query;
    let mut items = state.items.lock().expect("mutex was poisoned");
    update_scores(&query, &state.matcher, &mut items);
    let mut response_items = items.clone();
    if !query.is_empty() {
        response_items.sort();
        response_items.reverse();
    }
    Json(response_items)
}

#[derive(Deserialize)]
struct SubmitRequest {
    #[serde(rename = "selectedIds")]
    selected_ids: Vec<usize>,
}
async fn submit(State(state): State<AppState>, Json(req): Json<SubmitRequest>) {
    let items = state.items.lock().expect("mutex was poisoned");
    if !req.selected_ids.is_empty() {
        println!(
            "{}",
            items
                .iter()
                .filter(|item| req.selected_ids.contains(&item.id))
                .map(|item| item.key.clone())
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
    std::process::exit(0);
}
