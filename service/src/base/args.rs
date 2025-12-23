use clap::Parser;

#[derive(Parser, Debug)]
pub struct BaseArgs {
    #[clap(long, default_value_t = 6000, help = "RCSS player udp port")]
    pub player_port: u16,
    #[clap(long, default_value_t = 6001, help = "RCSS trainer udp port")]
    pub trainer_port: u16,
    #[clap(long, default_value_t = 6002, help = "RCSS coach udp port")]
    pub coach_port: u16,
    #[clap(long, default_value_t = true, help = "RCSS sync mode")]
    pub rcss_sync: bool,
    #[clap(long, default_value = "./log", help = "RCSS log directory")]
    pub rcss_log_dir: String,
}