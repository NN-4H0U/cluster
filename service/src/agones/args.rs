use clap::Parser;
use super::BaseArgs;

#[derive(Parser, Debug)]
pub struct AgonesArgs {
    #[clap(long, help = "Agones SDK port, default at 9357")]
    pub agones_port: Option<u16>,
    #[clap(long, help = "Agones SDK keep alive duration in seconds, default 30s")]
    pub agones_keep_alive: Option<u64>,

    #[clap(long, default_value_t = 5, help = "Agones health check interval in seconds")]
    pub health_check_interval: u64,
    #[clap(long, default_value_t = true, help = "Auto shutdown the server when the match is finished")]
    pub auto_shutdown_on_finish: bool,

    #[clap(flatten)]
    pub base_args: BaseArgs,
}
