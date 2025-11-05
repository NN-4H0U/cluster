mod ws;

use axum::Router;
use super::{AppState, Response};

pub fn route(path: &str, app_state: AppState) -> Router {
    let inner = Router::new()
        .merge(ws::route("/"))
        .with_state(app_state);

    if path == "/" {
        inner
    } else {
        Router::new().nest(path, inner)
    }
}