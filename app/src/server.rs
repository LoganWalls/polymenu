use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    Json, Router,
    extract::{Path, State},
    response::{Html, Result},
    routing::{get, post, put},
};
use serde::Deserialize;
use serde_json::Value;

use crate::{config::Config, item::Item, item_source::ItemSource};

#[derive(Clone)]
struct AppState {
    pub config: Config,
    pub items: Arc<Mutex<Vec<Item>>>,
}

impl AppState {
    pub fn new(config: Config) -> anyhow::Result<Self> {
        let mut source = ItemSource::new(&config);
        let items = source.get_items(HashMap::new())?;
        Ok(AppState {
            config,
            items: Arc::new(Mutex::new(items)),
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
        .route("/options", get(options))
        .route("/command/{name}", post(command))
        .route("/print", put(print_value))
        .route("/close", get(close))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    Ok(axum::serve(listener, app))
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn close() {
    std::process::exit(0);
}

async fn options(State(state): State<AppState>) -> Json<HashMap<String, Value>> {
    Json(state.config.options)
}

#[derive(Deserialize)]
struct PrintRequest {
    values: Vec<String>,
}

async fn print_value(Json(req): Json<PrintRequest>) {
    for v in req.values {
        println!("{v}")
    }
}

#[derive(Deserialize)]
struct CommandRequest {
    args: HashMap<String, String>,
}

async fn command(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(req): Json<CommandRequest>,
) -> Result<Json<Vec<Item>>> {
    let cmd = state
        .config
        .commands
        .get(&name)
        .unwrap_or_else(|| panic!("Command not found: {name}"));
    let _output = cmd
        .call(req.args)
        .unwrap_or_else(|_| panic!("Command failed: {cmd:?}"));
    // TODO: parse items from response
    Ok(Json(vec![]))
}
