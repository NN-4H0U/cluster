mod error;
mod http;
mod response;
mod ws;

use axum::Router;
use dashmap::DashMap;
use std::sync::{Arc, Weak};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::task::JoinHandle;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

pub use error::Error;
pub use response::Response;

use crate::Server;

#[derive(Clone)]
pub struct AppState {
    server: Arc<Server>,
    players: Arc<DashMap<Uuid, Weak<common::client::Client>>>,
}

pub async fn listen<A, S>(addr: A, shutdown: Option<S>) -> JoinHandle<Result<(), String>>
where
    A: ToSocketAddrs,
    S: Future<Output = ()> + Send + 'static,
{
    let state = AppState {
        server: Arc::new(Server::new().await),
        players: Arc::new(DashMap::new()),
    };

    state.server.spawn().await.expect("Failed to spawn server");

    let app = Router::new()
        .merge(http::route("/", state.clone()))
        .merge(ws::route("/player", state))
        .route_layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move {
        let serve = axum::serve(listener, app);

        match shutdown {
            Some(shutdown) => serve.with_graceful_shutdown(shutdown).await,
            None => serve.await,
        }
        .map_err(|e| e.to_string())
    })
}
