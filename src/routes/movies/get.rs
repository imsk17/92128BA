use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{entities::movie::Movie, AppState};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GetMovieParams {
    id: String,
}

/// Returns 404 in case if the movie with the ID is not present in the database.
#[axum::debug_handler]
pub async fn get_movie(
    Path(path_params): Path<GetMovieParams>,
    State(s): State<AppState>,
) -> Result<Json<Option<Movie>>, StatusCode> {
    let dbl = s.db.lock().expect("failed to get a lock on database");
    let mv = dbl.get(&path_params.id);
    if mv.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    return Ok(Json(mv.cloned()));
}
