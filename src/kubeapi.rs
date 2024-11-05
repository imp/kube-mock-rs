// use k8s::Resource as _;
// use kube::Resource as _;
use kube::ResourceExt;

use super::*;

use controller::Controller;
// use parser::Error;
use operations::Operation;
use parser::ParsedResource;
use resources::Item;
use verbs::Verb;

mod controller;
mod operations;
mod parser;
mod resources;
mod verbs;

#[derive(Debug)]
pub struct KubeApi {
    router: resources::Router,
    controllers: HashMap<api::TypeMeta, Box<dyn Controller>>,
}

impl KubeApi {
    pub fn new() -> Self {
        let builder = KubeMockBuilder::new().nodes(2);
        Self::from_builder(builder)
    }

    pub fn from_builder(builder: KubeMockBuilder) -> Self {
        let KubeMockBuilder {
            namespaces,
            nodes,
            pods,
        } = builder;

        let router = resources::Router::new();

        let namespaces = controller::Namespaces::with_inventory(namespaces).boxed();
        let nodes = controller::Nodes::with_inventory(nodes).boxed();
        let pods = controller::Pods::with_inventory(pods).boxed();

        let controllers = [namespaces, nodes, pods]
            .into_iter()
            .map(|controller| (controller.type_meta(), controller))
            .collect();

        Self {
            router,
            controllers,
        }
    }

    pub fn process_request(
        &mut self,
        parts: http::request::Parts,
        data: json::Value,
    ) -> kube::Result<Response<Body>> {
        let Some(operation) = self.router.operation(parts) else {
            let body = Body::from(b"404 page not found".to_vec());
            return response(body, 404);
        };

        let operation = match operation {
            Ok(operation) => operation,
            Err(status) => {
                let code = status.code;
                let body = serialize_to_body(status)?;
                return response(body, code);
            }
        };

        tracing::debug!(?operation);

        match self.process_operation(operation, data) {
            Ok((data, code)) => {
                let body = serialize_to_body(data)?;
                let code = code.as_u16() as i32;
                response(body, code)
            }
            Err(status) => {
                let code = status.code;
                let body = serialize_to_body(status)?;
                response(body, code)
            }
        }
    }

    pub fn process_operation(
        &mut self,
        operation: Operation,
        data: json::Value,
    ) -> Result<(json::Value, http::StatusCode), metav1::Status> {
        let meta = operation
            .type_meta()
            .inspect(|meta| tracing::debug!(?meta))
            .ok_or_else(metav1::Status::no_such_resource)?;

        let controller = self
            .controllers
            .get_mut(meta)
            .inspect(|controller| tracing::debug!(?controller))
            .ok_or_else(metav1::Status::no_such_resource)?;

        match operation {
            Operation::Create(object) => controller.create_op(object, data),
            Operation::Delete(object) => controller.delete_op(object, data),
            Operation::Get(object) => controller.get_op(object, data),
            Operation::List(object) => controller.list_op(object, data),
        }
    }
}

fn serialize_to_body<T: serde::Serialize>(data: T) -> kube::Result<Body> {
    json::to_vec(&data)
        .map(Body::from)
        .map_err(kube::Error::SerdeError)
}

fn from_json<K>(object: json::Value) -> Result<K, metav1::Status>
where
    K: kube::Resource + serde::de::DeserializeOwned,
{
    json::from_value(object).map_err(metav1::Status::bad_request)
}

fn response(body: Body, code: impl Into<Option<i32>>) -> kube::Result<Response<Body>> {
    Response::builder()
        .optionally_status(code.into())
        .body(body)
        .map_err(kube::Error::HttpError)
}
