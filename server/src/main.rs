mod entities;
mod error;
mod index;
mod market;

pub use error::AppError;

use std::sync::Arc;

use axum::{
    extract::State,
    routing::{any, get},
};
use log::warn;
use mongodb::{options::ClientOptions, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let ip = env_default("UNIKET_HOST", "0.0.0.0");
    let port = env_default("UNIKET_PORT", "3000");

    let db_address = env_default("UNIKET_DATABASE", "mongodb://localhost:27017");

    let client = Client::with_options(ClientOptions::parse_async(db_address).await?)?;

    let state = State(Arc::new(ServerState {
        db_conn: client.database("uniket"),
    }));

    let routes = axum::Router::new()
        .route("/", any(index::index))
        .route("/get_listings/:amt", get(market::list))
        .with_state(state);

    axum::Server::bind(&format!("{ip}:{port}").parse()?)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn env(p: &str) -> Option<String> {
    std::env::var(p).ok()
}

fn env_default(p: &str, default: &str) -> String {
    env(p).unwrap_or_else(|| {
        warn!("using default value ({default}) for {p}");
        default.to_string()
    })
}

struct ServerState {
    db_conn: mongodb::Database,
}
