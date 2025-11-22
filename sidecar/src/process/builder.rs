use std::io;
use std::process::Stdio;
use log::{error, trace};
use tokio::process::Command;

use super::*;

pub struct ServerProcessSpawner {
    pub pgm_name: &'static str,
    pub server_config:      ServerConfig,
    pub player_config:      PlayerConfig,
    pub csv_saver_config:   CsvSaverConfig,
}

impl ServerProcessSpawner {
    pub(super) async fn new(pgm_name: &'static str) -> Self {
        Self::validate(pgm_name).await;
        Self {
            pgm_name,
            server_config: ServerConfig::default(),
            player_config: PlayerConfig::default(),
            csv_saver_config: CsvSaverConfig::default(),
        }
    }

    async fn validate(pgm_name: &'static str) {
        let is_exist = {
            let mut validate_cmd = Command::new("whereis");
            validate_cmd.arg(pgm_name);
            let out = validate_cmd.output().await
                .expect("error calling whereis when creating RcssServer Process");

            let out = String::from_utf8_lossy(&out.stdout);
            trace!("RcssServer::validate: whereis returned: {out}");
            !out.ends_with(":")
        };

        if !is_exist {
            error!("RcssServer::validate panic: {} is not installed", pgm_name);
            panic!("{} is not installed", pgm_name);
        }
    }

    fn build_start_cmd(&self) -> Command {
        let mut cmd = Command::new(self.pgm_name);
        cmd.args(&self.server_config.to_args());
        cmd.args(&self.player_config.to_args());
        cmd.args(&self.csv_saver_config.to_args());
        cmd
    }

    pub async fn spawn(&self) -> Result<ServerProcess> {
        let mut cmd = self.build_start_cmd();
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd.spawn().map_err(|e| {
            match e.kind() {
                io::ErrorKind::WouldBlock => Error::MaxProcessReached(e),
                _ => Error::Io(e),
            }
        })?;

        ServerProcess::try_from(child).await
    }

    pub fn with_ports(&mut self, ports: u16, coach_port: u16, olcoach_port: u16) -> &mut Self {
        self.server_config_then(|c| {
            c.port          = Some(ports);
            c.coach_port    = Some(coach_port);
            c.olcoach_port  = Some(olcoach_port);
        })
    }

    pub fn with_server_config(&mut self, server_config: ServerConfig) -> &mut Self {
        self.server_config = server_config;
        self
    }
    pub fn with_player_config(&mut self, player_config: PlayerConfig) -> &mut Self {
        self.player_config = player_config;
        self
    }
    pub fn with_csv_saver_config(&mut self, csv_saver_config: CsvSaverConfig) -> &mut Self {
        self.csv_saver_config = csv_saver_config;
        self
    }

    #[inline]
    pub fn server_config_then(&mut self, f: impl FnOnce(&mut ServerConfig)) -> &mut Self {
        f(&mut self.server_config);
        self
    }
    #[inline]
    pub fn player_config_then(&mut self, f: impl FnOnce(&mut PlayerConfig)) -> &mut Self {
        f(&mut self.player_config);
        self
    }
    #[inline]
    pub fn csv_saver_config_then(&mut self, f: impl FnOnce(&mut CsvSaverConfig)) -> &mut Self {
        f(&mut self.csv_saver_config);
        self
    }

}