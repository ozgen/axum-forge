use axum::{Json, extract::State};

use crate::{
    error::AppError,
    modules::items::{dto::ItemResponse, repo},
    state::AppState,
};

pub async fn list_items(
    State(state): State<AppState>,
) -> Result<Json<Vec<ItemResponse>>, AppError> {
    let items = repo::list_items(&state.db)
        .await
        .map_err(AppError::internal_from_error)?;

    let response = items
        .into_iter()
        .map(ItemResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(response))
}
