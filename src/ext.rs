use super::*;

pub use status::StatusExt;

mod status;

pub(crate) trait SendResponseExt<T> {
    fn reply<E>(self, result: Result<T, E>)
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>;
}

impl<T> SendResponseExt<T> for tower_test::mock::SendResponse<T> {
    fn reply<E>(self, result: Result<T, E>)
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        match result {
            Ok(response) => self.send_response(response),
            Err(err) => self.send_error(err),
        }
    }
}

pub trait Optionally {
    fn optionally_status(self, code: Option<i32>) -> Self;
}

impl Optionally for http::response::Builder {
    fn optionally_status(self, code: Option<i32>) -> Self {
        if let Some(code) = code {
            self.status(code as u16)
        } else {
            self
        }
    }
}
