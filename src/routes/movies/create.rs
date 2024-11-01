use axum::{extract::State, http::StatusCode, Json};

use crate::{entities::movie::Movie, AppState};

/// There are two options in the way the payload could be send.
/// It could contain an ID or, it could just have all the other fields
/// except the ID. In the case id is not present, the id would have to be generated on the server
/// But in this case i will go ahead and assume the id will be provided by the caller
/// of this endpoint.
pub async fn create_movie(
    State(s): State<AppState>,
    Json(movie): Json<Movie>,
) -> (StatusCode, String) {
    let mut dbl =
        s.db.lock()
            .expect("failed to get a lock on the database mutex");
    dbl.insert(movie.id.clone(), movie.clone());
    return (StatusCode::CREATED, movie.id.clone());
}
