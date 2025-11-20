pub mod config;
pub mod error;

pub use config::*;
pub use error::*;

use std::io;
use std::process::Stdio;
use std::sync::{Arc, LazyLock};

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use log::{error, trace, info};

pub struct RcssServer {
    pub pgm_name: &'static str,
    pub server_config: ServerConfig,
    pub player_config: PlayerConfig,
    pub csv_saver_config: CsvSaverConfig,
}

pub struct ServerProcess {
    pub handle: JoinHandle<io::Result<std::process::ExitStatus>>,
    pub stdin_tx: mpsc::Sender<String>,
    pub stdout_rx: mpsc::Receiver<String>,
    pub stderr_rx: mpsc::Receiver<String>,
}

impl ServerProcess {
    async fn try_from(child: Child) -> Result<Self> {
        let (stdin_tx, mut stdin_rx) = mpsc::channel::<String>(100);
        let (stdout_tx, stdout_rx) = mpsc::channel::<String>(100);
        let (stderr_tx, stderr_rx) = mpsc::channel::<String>(100);

        let handle = tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            let mut child = child;

            let stdin = child.stdin.take().expect("Failed to open stdin");
            let stdout = child.stdout.take().expect("Failed to open stdout");
            let stderr = child.stderr.take().expect("Failed to open stderr");
            let mut stdout_reader = BufReader::new(stdout).lines();
            let mut stderr_reader = BufReader::new(stderr).lines();

            let mut stdin_writer = stdin;

            loop {
                tokio::select! {
                    status = child.wait() => {
                        info!("RcssServer child process exited with status: {:?}", status);
                        return status;
                    }

                    Some(line) = stdin_rx.recv() => {
                        if let Err(e) = stdin_writer.write_all(line.as_bytes()).await {
                            error!("Failed to write to stdin: {}", e);
                        }
                    }

                    // Read from stdout and send to channel.
                    result = stdout_reader.next_line() => {
                        match result {
                            Ok(Some(line)) => {
                                if stdout_tx.send(line).await.is_err() {
                                    // Channel closed, probably the receiver was dropped.
                                    break;
                                }
                            }
                            Ok(None) => break, // stdout closed
                            Err(e) => {
                                error!("Error reading from stdout: {}", e);
                                break;
                            }
                        }
                    }

                    // Read from stderr and send to channel.
                    result = stderr_reader.next_line() => {
                        match result {
                            Ok(Some(line)) => {
                                if stderr_tx.send(line).await.is_err() {
                                    break;
                                }
                            }
                            Ok(None) => break, // stderr closed
                            Err(e) => {
                                error!("Error reading from stderr: {}", e);
                                break;
                            }
                        }
                    }
                }
            }


            child.wait().await
        });

        todo!();
        Ok(Self {
            handle,
            stdin_tx,
            stdout_rx,
            stderr_rx,
        })
    }

    pub async fn terminate(&self) {
        todo!();
        self.stdin_tx.send("exit".to_string()).await.ok();
    }
}

impl RcssServer {
    pub async fn create(pgm_name: &'static str) -> Self {
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

    pub async fn start(&self) -> Result<ServerProcess> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_create() {
        RcssServer::create("rcssserver").await;
    }
}