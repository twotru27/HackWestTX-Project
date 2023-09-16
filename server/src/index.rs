use std::sync::Arc;

use axum::extract::State;

use crate::ServerState;

pub async fn index(State(arc): State<Arc<ServerState>>) {}
