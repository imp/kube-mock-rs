use super::*;

#[derive(Clone, Debug, serde::Serialize)]
pub struct ErrorResponse {
    status: Status,
    message: String,
    reason: Reason,
    #[serde(with = "http_serde::status_code")]
    code: http::StatusCode,
}

impl ErrorResponse {
    pub fn not_found<K>(name: impl ToString) -> Self
    where
        K: kube::Resource<DynamicType = ()>,
    {
        let name = name.to_string();
        let group = K::group(&());
        let plural = K::plural(&());
        let message = if group.is_empty() {
            format!("{plural} \"{name}\" not found")
        } else {
            format!("{plural}.{group} \"{name}\" not found")
        };
        Self {
            status: Status::Failure,
            message,
            reason: Reason::NotFound,
            code: http::StatusCode::NOT_FOUND,
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize)]
pub enum Status {
    Success,
    Failure,
}

#[derive(Clone, Copy, Debug, serde::Serialize)]
pub enum Reason {
    NotFound,
}

// ErrorResponse { status: "Failure", message: "deployments.apps \"engine1\" not found", reason: "NotFound", code: 404 }
