use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use axum::{
    Json, Router,
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Result},
    routing::{get, get_service, post, put},
};
use axum_extra::{
    TypedHeader,
    extract::cookie::{Cookie, CookieJar, SameSite},
    headers::{Authorization, authorization::Bearer},
};
use once_cell::sync::Lazy;
use rand::{Rng, distr::Alphanumeric};
use serde::Deserialize;
use serde_json::Value;
use tokio_util::sync::CancellationToken;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

use crate::{
    config::Config,
    expansion::expand_path,
    io::{DataParser, DataSourceKind},
};

pub static AUTH_TOKEN: Lazy<String> = Lazy::new(|| {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(48)
        .map(char::from)
        .collect()
});

const SESSION_COOKIE_NAME: &str = "session_id";

#[derive(Clone)]
struct AppState {
    pub config: Config,
    shutdown_token: CancellationToken,
}

impl AppState {
    pub fn new(config: Config, shutdown_token: CancellationToken) -> Self {
        AppState {
            config,
            shutdown_token,
        }
    }
}

pub async fn run(config: Config, shutdown_token: CancellationToken) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let ui_src = {
        let mut path = PathBuf::from(expand_path(config.src.as_ref().expect(
            "`app_src` must be provided either in your config file or as a CLI argument (neither was provided)"
        ))?);
        path.push("dist");
        path.into_os_string()
            .into_string()
            .expect("could not convert `app_src` to str")
    };
    let url = config.server_url();
    let ui_service = get_service(ServeDir::new(&ui_src));
    let mut mounted = Router::new();
    for (key, path) in config.mount.iter() {
        let expanded_path = dbg!(expand_path(path).context("failed to expand mount path")?);
        mounted = mounted.nest_service(
            &format!("/{key}"),
            get_service(ServeDir::new(expanded_path)),
        );
    }
    let api_routes = Router::new()
        .route("/options", get(options))
        .route("/input", get(read_input))
        .route("/command/{name}", post(command))
        .route("/print", put(print_value))
        .route("/close", put(close));

    let private_routes = Router::new()
        .nest("/api", api_routes)
        .nest("/files", mounted)
        .fallback_service(ui_service)
        .with_state(AppState::new(config, shutdown_token.clone()))
        .route_layer(axum::middleware::from_fn(require_auth));

    let app = Router::new()
        .route("/session", post(establish_session))
        .merge(private_routes)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(shutdown_token))
        .await
        .context("Problem starting server")
}

async fn shutdown_signal(token: CancellationToken) {
    token.cancelled().await;
}

async fn establish_session(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), StatusCode> {
    if bearer.token() != *AUTH_TOKEN {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }
    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, AUTH_TOKEN.clone());
    cookie.set_same_site(SameSite::Strict);
    cookie.set_http_only(true);
    Ok((jar.add(cookie), StatusCode::NO_CONTENT))
}

async fn require_auth(
    jar: CookieJar,
    bearer: Option<TypedHeader<Authorization<Bearer>>>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    let ok_header = bearer.as_ref().is_some_and(|h| h.token() == *AUTH_TOKEN);
    let ok_cookie = jar
        .get(SESSION_COOKIE_NAME)
        .is_some_and(|c| c.value() == *AUTH_TOKEN);
    if !(ok_header || ok_cookie) {
        return (StatusCode::UNAUTHORIZED, "unauthorized").into_response();
    }
    next.run(req).await
}

async fn read_input(State(state): State<AppState>) -> Json<Vec<Value>> {
    let parser: DataParser = state.config.into();
    Json(parser.parse(HashMap::new()).unwrap())
}

async fn close(State(state): State<AppState>) {
    state.shutdown_token.cancel();
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
