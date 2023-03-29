use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};

use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use backend::prelude::*;
use backend::utils;

#[tokio::main]
async fn main() {
    // let env = fs::read_to_string(".env").expect("Unable to read .env file");
    // let (key, database_url) = env.split_once("=").expect("Invalid .env file");
    // assert_eq!(key.trim(), "DATABASE_URL");
    let env = utils::read_env().expect("Unable to read .env file");
    let database_url = env.get("DATABASE_URL").expect("DATABASE_URL not found in .env file");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=trace")
                .unwrap_or_else(|_| "example_tracing_aka_loging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to connect to Postgres.")
        .unwrap();

    let app = Router::new()
        .route("/hello", get(hello))
        .route("/users", get(user_ctrl::all_users))
        .route("/users/:id", get(user_ctrl::user))
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3334));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello, World!"
}
