#[cfg(target_os = "windows")]
compile_error!("This program currently not supported on Windows.");

mod process;
mod coach;
mod test;
mod service;

use std::env;
use std::time::Duration;

use agones::Sdk;

use env_logger;
use log::{debug, error, info};

use coach::command;

pub const RCSS_PROCESS_NAME: &'static str = "rcssserver";
pub const PEER_IP: std::net::IpAddr = std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);

#[tokio::main]
async fn main() {
    unsafe { env::set_var("RUST_LOG", "trace") };
    env_logger::init();

    let mut sdk = Sdk::new(None, None).await.expect("failed to connect to SDK server");

    // let builder = coached_process::CoachedProcess::spawner().await;
    // let mut server = builder.spawn().await.unwrap();
    //
    // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    //
    // let ret = server.shutdown().await.unwrap();
}

pub const STATUS_CHECK_INT: Duration = Duration::from_secs(2);
async fn run_server(sdk: Sdk) -> () {

    let builder = service::CoachedProcess::spawner().await;
    let server = builder.spawn().await.unwrap();
    let coach_caller = server.coach().caller::<command::CheckBall>();

    let game_status_task = {
        let caller = coach_caller.clone();
        let sdk_ = sdk.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(STATUS_CHECK_INT);

            while let Ok(Ok((time, pos))) = caller.call(command::CheckBall).await {
                debug!("Ball at time {} pos {:?}", time, pos);

                if sdk_.health_check().send(()).await.is_err() {
                    error!("Failed to send health ping to Agones SDK");
                    return;
                }

                interval.tick().await;
            }
        })
    };


    game_status_task.await.unwrap();

    {
        let mut sdk = sdk;
        server.shutdown().await.unwrap();
        sdk.shutdown().await.unwrap();
    }

}