use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt;
use std::str;

use k8s_openapi_ext as k8s;

use http::{Request, Response};
use kube::api;
use kube::client::Body;
use serde_json as json;
use tokio::task;
use tower_test::mock;

use k8s::appsv1;
use k8s::batchv1;
use k8s::corev1;
use k8s::metav1;
use k8s::rbacv1;

use ext::SendResponseExt as _;

const DEFAULT_NS: &str = "default";

mod builder;
mod ext;
mod kubeapi;

#[derive(Debug)]
pub struct KubeMock {
    handle: mock::Handle<Request<Body>, Response<Body>>,
    kubeapi: kubeapi::KubeApi,
}

impl KubeMock {
    pub fn pair() -> (kube::Client, Self) {
        let (service, handle) = mock::pair();
        let client = kube::Client::new(service, DEFAULT_NS);
        let kubeapi = kubeapi::KubeApi::new();
        (client, Self { handle, kubeapi })
    }

    pub async fn serve(mut self) {
        while let Some((request, send_response)) = self.handle.next_request().await {
            let method = request.method();
            let uri = request.uri();
            tracing::debug!(%method, %uri, "MOCK");
            let result = self
                .kubeapi
                .parse_request(request)
                .await
                .inspect(|(path, verb, _)| tracing::debug!(?verb, ?path, "KUBEAPI PARSE"))
                .and_then(|(path, verb, data)| self.kubeapi.dispatch(path, verb, data))
                .and_then(serialize_to_body)
                .and_then(|body| {
                    Response::builder()
                        .body(body)
                        .map_err(kube::Error::HttpError)
                });
            send_response.reply(result);
        }
    }

    pub async fn run(self) -> task::JoinHandle<()> {
        tokio::spawn(self.serve())
    }
}

fn serialize_to_body(data: json::Value) -> kube::Result<Body> {
    json::to_vec(&data)
        .map(Body::from)
        .map_err(kube::Error::SerdeError)
}
