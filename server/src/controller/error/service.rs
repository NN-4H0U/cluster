use super::Response;
use axum::http::StatusCode;


pub struct ServiceError<'a>(pub &'a service::Error);

impl<'a> ServiceError<'a> {
    pub fn status_code(&self) -> StatusCode {
        match &self.0 {
            service::Error::ServerNotRunning { status: _ } => StatusCode::OK,
            service::Error::ServerStillRunningToRestart => StatusCode::OK,
        }
    }
}

impl<'a> From<ServiceError<'a>> for Response {
    fn from(value: ServiceError<'a>) -> Self {
        match &value.0 {
            service::Error::ServerNotRunning { status: _ } => {
                Response::error("ServerNotRunning", "The process server is not running.")
            }
            service::Error::ServerStillRunningToRestart => Response::error(
                "ServerStillRunningToRestart",
                "The process server is still running, try to call with `force=true` to ignore.",
            ),
        }
    }
}
