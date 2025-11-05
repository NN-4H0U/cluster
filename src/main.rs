mod service;
mod controller;
mod model;

#[tokio::main]
async fn main() {
    controller::listen("0.0.0.0:55555").await;
}
