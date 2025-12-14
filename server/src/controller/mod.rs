mod error;
mod http;
mod response;
mod ws;

use axum::Router;
use dashmap::DashMap;
use std::sync::{Arc, Weak};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::task::JoinHandle;

pub use error::Error;
pub use response::Response;

use tower_http::trace::TraceLayer;
use uuid::Uuid;

use service::Service;

#[derive(Clone)]
pub struct AppState {
    service: Arc<Service>,
    players: Arc<DashMap<Uuid, Weak<common::client::Client>>>,
}

pub async fn listen<A: ToSocketAddrs>(addr: A) -> JoinHandle<Result<(), String>> {
    let state = AppState {
        service: Arc::new(Service::new().await),
        players: Arc::new(DashMap::new()),
    };

    state.service.spawn().await;

    let app = Router::new()
        .merge(http::route("/", state.clone()))
        .merge(ws::route("/player", state))
        .route_layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move { axum::serve(listener, app).await.map_err(|e| e.to_string()) })
}
