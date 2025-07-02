use std::collections::HashMap;

use anyhow::Context;
use axum::{
    Json, Router,
    extract::{Path, State},
    response::{Html, Result},
    routing::{get, post, put},
};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    config::Config,
    io::{DataParser, DataSourceKind},
};

#[derive(Clone)]
struct AppState {
    pub config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        AppState { config }
    }
}

pub async fn run(
    config: Config,
) -> anyhow::Result<axum::serve::Serve<tokio::net::TcpListener, Router, Router>> {
    tracing_subscriber::fmt::init();
    let state = AppState::new(config);
    let app = Router::new()
        .route("/", get(root))
        .route("/options", get(options))
        .route("/input", get(read_input))
        .route("/command/{name}", post(command))
        .route("/print", put(print_value))
        .route("/close", put(close))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    Ok(axum::serve(listener, app))
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn read_input(State(state): State<AppState>) -> Json<Vec<Value>> {
    let parser: DataParser = state.config.into();
    Json(parser.parse(HashMap::new()).unwrap())
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
) -> Result<Json<Vec<Value>>> {
    let cmd = state
        .config
        .commands
        .get(&name)
        .with_context(|| format!("Command not found: {name}"))
        .unwrap();
    let data = DataParser::new(
        DataSourceKind::Command(cmd.clone()),
        cmd.output_format,
        None,
    )
    .parse(req.args)
    .with_context(|| format!("Could not parse output for command: {name}"))
    .unwrap();
    Ok(Json(data))
}
