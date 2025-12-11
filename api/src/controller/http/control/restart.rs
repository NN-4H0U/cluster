use axum::extract::State;
use axum::{routing, Json, Router};
use serde::Deserialize;
use super::{AppState, Response};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostRequest {
    #[serde(default)]
    pub force: bool,
}
async fn post(
    State(state): State<AppState>,
    Json(req): Json<PostRequest>,

) -> Response {
    let res = state.sidecar.restart(req.force).await;
    match res {
        Ok(_) => Response::success::<()>(None),
        Err(e) => Response::error("Restart Failed", &e.to_string()),
    }
}

pub fn route(path: &str) -> Router<AppState> {
    Router::new()
        .route(path, routing::post(post))
}
