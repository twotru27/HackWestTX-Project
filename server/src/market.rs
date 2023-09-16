use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use futures_core::*;
use mongodb::{
    bson::{Document, RawDocumentBuf},
    options::FindOptions,
    Cursor,
};
use serde::{Deserialize, Serialize};

use crate::{entities::SaleListing, AppError, ServerState};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse {
    listings: Vec<RawDocumentBuf>,
}

pub async fn list(State(arc): State<Arc<ServerState>>) -> Result<Json<ListResponse>, AppError> {
    let conn = &arc.db_conn;
    let mut collection: Cursor<SaleListing> = conn.collection("listings").find(None, None).await?;

    let mut listings = Vec::with_capacity(collection.size_hint().0);

    while let Ok(true) = collection.advance().await {
        listings.push(collection.current().to_raw_document_buf());
    }

    Ok(Json(ListResponse { listings }))
}
