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

pub trait Optionally: Sized {
    fn optionally_status(self, _code: Option<i32>) -> Self {
        self
    }
    fn optionally_within(self, _ns: Option<&str>) -> Self {
        self
    }
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

impl Optionally for api::DynamicObject {
    fn optionally_within(self, ns: Option<&str>) -> Self {
        if let Some(ns) = ns {
            self.within(ns)
        } else {
            self
        }
    }
}
