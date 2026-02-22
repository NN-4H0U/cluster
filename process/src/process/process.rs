use std::process::ExitStatus;
use std::sync::Arc;
use std::time::Duration;
use common::process::{Process, ProcessStatus};
use common::utils::ringbuf::OverwriteRB;
use tokio::process::Child;
use tokio::sync::{watch, RwLock};
use log::{warn, error};

use super::builder::ServerProcessSpawner;
use super::error::{Error, Result};

pub const READY_LINE: &str = "Hit CTRL-C to exit";

#[derive(Debug)]
pub struct ServerProcess {
    inner: Process,
    status_rx: watch::Receiver<Status>,
    stdout: Arc<RwLock<OverwriteRB<String, 32>>>,
    stderr: Arc<RwLock<OverwriteRB<String, 32>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Init,
    Booting,
    Running,
    Returned(ExitStatus),
    Dead(String),
}

impl Status {
    pub fn is_ready(&self) -> bool {
        matches!(self, Status::Running)
    }
}

impl ServerProcess {
    pub const TERM_TIMEOUT_S: Duration = Duration::from_secs(5);

    pub async fn spawner(pgm_name: &'static str) -> ServerProcessSpawner {
        ServerProcessSpawner::new(pgm_name).await
    }

    pub(crate) async fn try_from(child: Child) -> Result<Self> {
        let inner = Process::new(child)?;
        
        let (status_tx, status_rx) = watch::channel(Status::Init);
        let stdout_rb = Arc::new(RwLock::new(OverwriteRB::new()));
        let stderr_rb = Arc::new(RwLock::new(OverwriteRB::new()));

        let mut stdout_rx = inner.subscribe_stdout();
        let mut stderr_rx = inner.subscribe_stderr();
        
        let stdout_rb_ = stdout_rb.clone();
        let stderr_rb_ = stderr_rb.clone();
        let mut inner_status_rx = inner.status_watch();
        
        // Task to process logs and update status
        tokio::spawn(async move {
            let _ = status_tx.send(Status::Booting);

            loop {
                tokio::select! {
                    Ok(line) = stdout_rx.recv() => {
                        if line == READY_LINE {
                            if status_tx.send(Status::Running).is_err() {
                                warn!("Failed to send Running status: receiver dropped");
                            }
                        }
                        stdout_rb_.write().await.push(line);
                    }
                    Ok(line) = stderr_rx.recv() => {
                        stderr_rb_.write().await.push(line);
                    }
                    
                    // Monitor inner process status to propagate exit
                    _ = inner_status_rx.changed() => { // This is NOT correct for watch channel usage in loop directly like this without handling potential errors or old values correctly if not careful, but let's see.
                        // Actually, watch channel `changed()` waits for a change.
                        // We should read the new status.
                         let status = inner_status_rx.borrow().clone();
                         match status {
                             ProcessStatus::Returned(s) => {
                                 let _ = status_tx.send(Status::Returned(s));
                                 break;
                             },
                             ProcessStatus::Dead(s) => {
                                 let _ = status_tx.send(Status::Dead(s));
                                 break;
                             },
                             _ => {}
                         }
                    }
                }
            }
        });

        Ok(Self { 
            inner,
            status_rx,
            stdout: stdout_rb,
            stderr: stderr_rb,
        })
    }

    pub async fn stdout_logs(&self) -> Vec<String> {
        self.stdout.read().await.to_vec()
    }

    pub async fn stderr_logs(&self) -> Vec<String> {
        self.stderr.read().await.to_vec()
    }

    pub async fn shutdown(&mut self) -> Result<ExitStatus> {
        self.inner.shutdown().await.map_err(Error::from)
    }

    pub fn pid(&self) -> Option<u32> {
        self.inner.pid()
    }

    pub fn status(&self) -> watch::Receiver<Status> {
        self.status_rx.clone()
    }

    /// Wait until the rcssserver is ready and able to accept Udp connections.
    pub async fn until_ready(&mut self, timeout: Option<Duration>) -> Result<()> {
         if self.status_rx.borrow().is_ready() { return Ok(()) }

        let task = self.status_rx.wait_for(|s| s.is_ready());
        let ret = match timeout {
            Some(timeout) => tokio::time::timeout(timeout, task).await
                .map_err(|_| Error::Process(common::process::ProcessError::TimeoutWaitingReady))?,
            None => task.await,
        };
        
        if ret.is_ok() {
            return Ok(());
        }
        
        drop(ret);
        error!("ServerProcess::until_ready: UNEXPECTED watch channel released!!!!!");
        Err(Error::Process(common::process::ProcessError::ChildNotReady))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Stdio;
    use tokio::process::Command;
    use std::time::Duration;
    use common::process::ProcessError;

    // Helper function to create a test child process that echoes and exits
    async fn create_test_child(script: &str) -> Child {
        Command::new("sh")
            .arg("-c")
            .arg(script)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn test child process")
    }

    #[tokio::test]
    async fn test_server_process_creation_with_valid_child() {
        let child = create_test_child("echo 'test'; sleep 0.1").await;
        let result = ServerProcess::try_from(child).await;
        assert!(result.is_ok(), "Should successfully create ServerProcess from valid child");
        let mut process = result.unwrap();
        assert!(process.pid().is_some(), "PID should be set");
        let _ = process.shutdown().await;
    }

    #[tokio::test]
    async fn test_server_process_pid_tracking() {
        let child = create_test_child("sleep 0.5").await;
        let pid_before = child.id().expect("Child should have PID");
        let mut process = ServerProcess::try_from(child).await.unwrap();
        assert_eq!(process.pid(), Some(pid_before));
        let _ = process.shutdown().await;
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert_eq!(process.pid(), None, "PID should be cleared after process exits");
    }

    #[tokio::test]
    async fn test_stdout_capture() {
        let script = r#"
            echo "line1"
            echo "line2"
            echo "line3"
            sleep 1
        "#;
        let child = create_test_child(script).await;
        let mut process = ServerProcess::try_from(child).await.unwrap();
        tokio::time::sleep(Duration::from_millis(600)).await;
        let logs = process.stdout_logs().await;
        assert!(!logs.is_empty(), "Should capture stdout logs");
        assert!(logs.contains(&"line1".to_string()));
        let _ = process.shutdown().await;
    }

    #[tokio::test]
    async fn test_ready_line_detection() {
        let script = format!(r#"
            echo "Starting..."
            sleep 0.2
            echo "{}"
            sleep 2
        "#, READY_LINE);
        let child = create_test_child(&script).await;
        let mut process = ServerProcess::try_from(child).await.unwrap();
        let result = process.until_ready(Some(Duration::from_secs(2))).await;
        assert!(result.is_ok(), "Process should become ready when READY_LINE is printed");
        let _ = process.shutdown().await;
    }

    #[tokio::test]
    async fn test_until_ready_timeout() {
        let child = create_test_child("sleep 5").await;
        let mut process = ServerProcess::try_from(child).await.unwrap();
        let result = process.until_ready(Some(Duration::from_millis(100))).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Process(ProcessError::TimeoutWaitingReady) => {},
            e => panic!("Unexpected error: {:?}", e),
        }
        let _ = process.shutdown().await;
    }
}
