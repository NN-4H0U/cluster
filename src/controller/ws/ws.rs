use axum::{routing, Router, response::Response as AxumResponse};
use axum::extract::{Path, State, WebSocketUpgrade};

use super::{Response, AppState};

async fn upgrade(
    State(s): State<AppState>,
    ws: WebSocketUpgrade, Path(path): Path<String>
) -> AxumResponse {
    todo!()
}

pub fn route(path: &str) -> Router<AppState> {
    let path = if path == "/" { "/{room_id}" } else { &format!("{path}/{{room_id}}") };
    Router::new()
        .route(path, routing::get(upgrade))
}
