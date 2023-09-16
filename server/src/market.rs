use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use mongodb::{bson::Document, options::FindOptions, Cursor};

use crate::{AppError, ServerState};

pub struct ListResponse {}

pub async fn list(State(arc): State<Arc<ServerState>>) -> Result<ListResponse, AppError> {
    let conn = &arc.db_conn;
    let collection: Cursor<SaleListing> = conn.collection("listings").find(None, None).await?;

    Ok(ListResponse {})
}
