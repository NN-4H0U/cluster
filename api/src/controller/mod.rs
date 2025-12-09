mod error;
mod response;
mod http;
mod ws;

use std::sync::{Arc, Weak};
use axum::Router;
use dashmap::DashMap;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::task::JoinHandle;

pub use response::Response;
pub use error::Error;

use tower_http::trace::TraceLayer;
use uuid::Uuid;
use sidecar::Sidecar;

#[derive(Clone)]
pub struct AppState {
    sidecar: Arc<Sidecar>,
    players: Arc<DashMap<Uuid, Weak<common::client::Client>>>
}

pub async fn listen<A: ToSocketAddrs>(
    addr: A,
) -> JoinHandle<Result<(), String>> {
    let state = AppState { sidecar: Arc::new(Sidecar::new().await), players: Arc::new(DashMap::new()) };

    state.sidecar.spawn().await;

    let app = Router::new()
        .merge(http::route("/", state.clone()))
        .merge(ws::route("/player", state))
        .route_layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move {
        axum::serve(listener, app).await.map_err(|e| e.to_string())
    })
}
 
