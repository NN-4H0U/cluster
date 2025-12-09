use std::borrow::Cow;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use super::Response;

pub struct SidecarError<'a>(pub &'a sidecar::Error);

impl<'a> SidecarError<'a> {
    pub fn status_code(&self) -> StatusCode {
        match &self.0 {
            sidecar::Error::ServerNotRunning { status: _ } => StatusCode::OK,
        }
    }
}

impl<'a> From<SidecarError<'a>> for Response {
    fn from(value: SidecarError<'a>) -> Self {
        match &value.0 {
            sidecar::Error::ServerNotRunning { status: _ } => {
                Response::error("ServerNotRunning", "The sidecar server is not running.")
            }
        }
    }
}