use http::Method;

use super::*;

#[derive(Debug)]
pub enum Verb {
    Create,
    Get,
    List,
    Watch,
    Delete,
    DeleteCollection,
    Update,
    Patch,
}

impl Verb {
    pub fn from_request(request: &Request<Body>) -> Self {
        match *request.method() {
            Method::GET => Self::Get,
            Method::POST => Self::Update,
            Method::PUT => Self::Create,
            Method::DELETE => Self::Delete,
            Method::PATCH => Self::Update,
            _ => panic!("unsupported method"),
        }
    }
}
