use super::Response;
use axum::http::StatusCode;

pub struct ServerError<'a>(pub &'a crate::Error);

impl<'a> ServerError<'a> {
    pub fn status_code(&self) -> StatusCode {
        match &self.0 {
            crate::Error::ServerNotRunning { status: _ } => StatusCode::OK,
            crate::Error::ServerStillRunningToRestart => StatusCode::OK,
        }
    }
}

impl<'a> From<ServerError<'a>> for Response {
    fn from(value: ServerError<'a>) -> Self {
        match &value.0 {
            crate::Error::ServerNotRunning { status: _ } => {
                Response::error("ServerNotRunning", "The server server is not running.")
            }
            crate::Error::ServerStillRunningToRestart => Response::error(
                "ServerStillRunningToRestart",
                "The server server is still running, try to call with `force=true` to ignore.",
            ),
        }
    }
}
