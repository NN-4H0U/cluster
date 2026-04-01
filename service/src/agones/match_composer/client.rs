use std::time::Duration;
use log::{debug, info, warn};
use reqwest::Client;

use super::error::Error;
use super::response::StatusResponse;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
const START_MAX_RETRIES: u32 = 3;
const START_RETRY_BASE: Duration = Duration::from_secs(1);

#[derive(Clone, Debug)]
pub struct MatchComposerClient {
    client: Client,
    base_url: String,
}

impl MatchComposerClient {
    pub fn new(port: u16) -> Self {
        let client = Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()
            .expect("failed to build reqwest client");

        let base_url = format!("http://127.0.0.1:{port}");

        Self { client, base_url }
    }

    /// POST /start with built-in retry logic for sidecar startup race.
    /// Retries up to 3 times with exponential backoff (1s, 2s, 4s).
    pub async fn start(&self) -> Result<(), Error> {
        let url = format!("{}/start", self.base_url);

        let mut last_err = None;
        for attempt in 0..START_MAX_RETRIES {
            if attempt > 0 {
                let backoff = START_RETRY_BASE * 2u32.pow(attempt - 1);
                info!(
                    "[MatchComposerClient] start retry {}/{} after {}ms",
                    attempt + 1,
                    START_MAX_RETRIES,
                    backoff.as_millis()
                );
                tokio::time::sleep(backoff).await;
            }

            match self.client.post(&url).header("content-type", "application/json").body("{}").send().await {
                Ok(resp) => {
                    return check_response(resp).await.map(|_| ());
                }
                Err(e) if e.is_connect() => {
                    warn!(
                        "[MatchComposerClient] start attempt {}/{} connection failed: {e}",
                        attempt + 1,
                        START_MAX_RETRIES
                    );
                    last_err = Some(Error::Connection(e));
                    continue;
                }
                Err(e) => return Err(Error::Connection(e)),
            }
        }

        Err(last_err.unwrap_or_else(|| {
            Error::RequestFailed {
                status: 0,
                body: "max retries exceeded".into(),
            }
        }))
    }

    /// POST /stop
    pub async fn stop(&self) -> Result<(), Error> {
        let url = format!("{}/stop", self.base_url);
        let resp = self.client.post(&url).send().await.map_err(Error::Connection)?;
        check_response(resp).await.map(|_| ())
    }

    /// POST /restart (reserved for future use)
    pub async fn restart(&self) -> Result<(), Error> {
        let url = format!("{}/restart", self.base_url);
        let resp = self
            .client
            .post(&url)
            .header("content-type", "application/json")
            .body("{}")
            .send()
            .await
            .map_err(Error::Connection)?;
        check_response(resp).await.map(|_| ())
    }

    /// GET /status
    pub async fn status(&self) -> Result<StatusResponse, Error> {
        let url = format!("{}/status", self.base_url);
        let resp = self.client.get(&url).send().await.map_err(Error::Connection)?;
        let resp = check_response(resp).await?;
        resp.json::<StatusResponse>().await.map_err(Error::DeserializeFailed)
    }
}

async fn check_response(resp: reqwest::Response) -> Result<reqwest::Response, Error> {
    let status = resp.status();
    if status.is_success() {
        debug!("[MatchComposerClient] Response OK ({status})");
        Ok(resp)
    } else {
        let body = resp.text().await.unwrap_or_default();
        Err(Error::RequestFailed {
            status: status.as_u16(),
            body,
        })
    }
}
