mod server;
mod coach;
mod sidecar;

#[tokio::main]
async fn main() {
    let server = server::RcssServer::create("rcssserver").await;
}
