use uuid::Uuid;
use futures::StreamExt;

use axum::{routing, Router, response::Response as AxumResponse};
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::extract::ws::WebSocket;

use super::AppState;

async fn upgrade(
    State(s): State<AppState>,
    ws: WebSocketUpgrade, Path((room_id, team_name)): Path<(Uuid, String)>
) -> AxumResponse {
    ws.on_upgrade(move |socket| async move {
        handle_upgrade(socket, room_id, team_name).await
    })
}

async fn handle_upgrade(socket: WebSocket, room_id: Uuid, team_name: String) -> () {
    let (mut tx, mut rx) = socket.split();
    todo!()
}

pub fn route(path: &str) -> Router<AppState> {
    let path =
        if path == "/" { "/{room_id}/{team_name}" }
        else { &format!("{path}/{{room_id}}/{{team_name}}") };

    Router::new().route(path, routing::get(upgrade))
}
