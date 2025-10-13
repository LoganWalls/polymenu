use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use axum::{
    Json, Router,
    extract::{Path, State},
    response::Result,
    routing::{get, get_service, post, put},
};
use serde::Deserialize;
use serde_json::Value;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::EnvFilter;

use crate::{
    config::Config,
    expansion::expand_path,
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
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let ui_src = {
        let mut path = PathBuf::from(expand_path(config.app_src.as_ref().expect(
            "`app_src` must be provided either in your config file or as a CLI argument (neither was provided)"
        ))?);
        path.push("dist");
        path.into_os_string()
            .into_string()
            .expect("could not convert `app_src` to str")
    };
    let url = config.server_url();
    let ui_service = get_service(
        ServeDir::new(&ui_src).not_found_service(ServeFile::new(format!("{ui_src}/index.html"))),
    );
    let api_routes = Router::new()
        .route("/options", get(options))
        .route("/input", get(read_input))
        .route("/command/{name}", post(command))
        .route("/print", put(print_value))
        .route("/close", put(close));
    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(ui_service)
        .with_state(AppState::new(config))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    Ok(axum::serve(listener, app))
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
