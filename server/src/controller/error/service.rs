use super::Response;
use axum::http::StatusCode;

pub struct ServiceError<'a>(pub &'a service::Error);

impl<'a> ServiceError<'a> {
    pub fn status_code(&self) -> StatusCode {
        match &self.0 {
            service::Error::ServerNotRunning { status: _ } => StatusCode::OK,
            service::Error::ServerStillRunningToSpawn => StatusCode::OK,
            service::Error::Timeout { op: _ } => StatusCode::REQUEST_TIMEOUT,
        }
    }
}

impl<'a> From<ServiceError<'a>> for Response {
    fn from(value: ServiceError<'a>) -> Self {
        match &value.0 {
            service::Error::ServerNotRunning { status: _ } => {
                Response::error("ServerNotRunning", "The process server is not running.")
            }
            service::Error::ServerStillRunningToSpawn => Response::error(
                "ServerStillRunningToSpawn", &value.0.to_string(),
            ),
            service::Error::Timeout { op } => {
                Response::error("Timeout", &value.0.to_string())
            },
        }
    }
}
