mod create;

use uuid::Uuid;
use axum::extract::{Query, State};
use axum::Router;
use log::debug;
use serde::Deserialize;
use super::{AppState, Response};

#[derive(Deserialize, Debug)]
pub struct DeleteRequest {
    pub room_id: Uuid,
}

pub async fn delete(
    State(s): State<AppState>,
    Query(req): Query<DeleteRequest>
) -> Response {
    let ret = s.cluster.drop_room(req.room_id);
    debug!("/room/delete: dropping Room[{}], {ret:?}", req.room_id);
    match ret {
        Ok(_)  => Response::success::<()>(None),
        Err(_) => Response::code_u16(404),
    }
}

pub fn route(path: &str) -> Router<AppState> {
    let inner = Router::new()
        .route("/", axum::routing::delete(delete))
        .merge(create::route("/create"))
        // .merge(detail::route("/detail"))
    ;
    if path == "/" {
        inner
    } else {
        Router::new().nest(path, inner)
    }
}