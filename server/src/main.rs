mod entities;
mod error;
mod index;
mod market;

pub use error::AppError;

use std::{fmt::Display, sync::Arc};

use axum::{
    extract::State,
    routing::{any, get},
};
use log::warn;
use mongodb::{
    options::{AuthMechanism, ClientOptions, Credential},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let ip = env_default("UNIKET_HOST", "0.0.0.0");
    let port = env_default("UNIKET_PORT", "3000");

    let token = env("UNIKET_DATABASE_TOKEN").expect("token to be provided");
    let db_address = env_default(
        "UNIKET_DATABASE",
        format!(
            "mongodb+srv://uniket:{token}@uniket.8j6mykm.mongodb.net/?retryWrites=true&w=majority"
        ),
    );

    let client = Client::with_options({
        let mut options = ClientOptions::parse_async(db_address).await?;
        options.credential = Some(
            Credential::builder()
                .mechanism(AuthMechanism::Plain)
                .password(token)
                .build(),
        );
        options
    })?;

    let state = Arc::new(ServerState {
        db_conn: client.database("uniket"),
    });

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

fn env_default(p: &str, default: impl Into<String> + Display) -> String {
    env(p).unwrap_or_else(|| {
        warn!("using default value ({default}) for {p}");
        default.into()
    })
}

struct ServerState {
    db_conn: mongodb::Database,
}
