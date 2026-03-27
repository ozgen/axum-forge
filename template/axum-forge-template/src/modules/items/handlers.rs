use axum::{extract::State, Json};

use crate::{
    error::AppError,
    modules::items::dto::ItemResponse,
    state::AppState,
};

pub async fn list_items(
    State(state): State<AppState>,
) -> Result<Json<Vec<ItemResponse>>, AppError> {
    let items = state
        .items_repo
        .list_items()
        .await
        .map_err(AppError::internal_from_error)?;

    let response = items
        .into_iter()
        .map(ItemResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(response))
}