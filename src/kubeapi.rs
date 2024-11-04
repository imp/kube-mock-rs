// use k8s::Resource as _;
// use kube::Resource as _;
use kube::ResourceExt;

use super::*;

use controller::Controller;
// use parser::Error;
use parser::ParsedResource;
use verbs::Verb;

mod controller;
mod parser;
mod resources;
mod verbs;

// #[derive(Debug)]
pub struct KubeApi {
    resources: HashMap<String, api::ApiResource>,

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

        let resources = resources::api_resources();

        let namespaces = controller::Namespaces::with_inventory(namespaces).boxed();
        let nodes = controller::Nodes::with_inventory(nodes).boxed();
        let pods = controller::Pods::with_inventory(pods).boxed();

        let controllers = [namespaces, nodes, pods]
            .into_iter()
            .map(|controller| (controller.type_meta(), controller))
            .collect();

        Self {
            resources,
            controllers,
        }
    }

    pub async fn process(&mut self, request: Request<Body>) -> kube::Result<Response<Body>> {
        let (resource, verb, data) = self.parse_request(request).await?;
        tracing::debug!(?verb, ?resource, "KUBEAPI PARSE");
        let response = match self.dispatch(resource, verb, data) {
            Ok((data, code)) => {
                let body = serialize_to_body(data)?;
                Response::builder()
                    .status(code)
                    .body(body)
                    .map_err(kube::Error::HttpError)?
            }
            Err(status) => {
                let code = status.code;
                let body = serialize_to_body(status)?;
                Response::builder()
                    .optionally_status(code)
                    .body(body)
                    .map_err(kube::Error::HttpError)?
            }
        };
        Ok(response)
    }

    pub fn dispatch(
        &mut self,
        resource: ParsedResource,
        verb: Verb,
        data: json::Value,
    ) -> Result<(json::Value, http::StatusCode), metav1::Status> {
        let meta = self
            .get_type_meta(&resource)
            .ok_or_else(metav1::Status::no_such_resource)?;

        tracing::debug!(?meta);
        self.controllers
            .get_mut(&meta)
            .inspect(|controller| tracing::debug!(?controller))
            .ok_or_else(metav1::Status::no_such_resource)?
            .handle(resource, verb, data)
            .map(|data| (data, http::StatusCode::OK))
    }

    pub async fn parse_request(
        &self,
        request: Request<Body>,
    ) -> kube::Result<(ParsedResource, Verb, json::Value)> {
        let method = request.method();
        let uri = request.uri();
        let parsed_path = uri.path().parse::<ParsedResource>()?;
        let verb = parsed_path.verb(method).unwrap(); // metav1::Status::method_not_allowed
        let body = request.into_body();
        let bytes = body.collect_bytes().await?;
        let value = if bytes.is_empty() {
            json::Value::default()
        } else {
            json::from_slice(&bytes).map_err(kube::Error::SerdeError)?
        };
        Ok((parsed_path, verb, value))
    }

    fn get_type_meta(&self, path: &ParsedResource) -> Option<api::TypeMeta> {
        self.resources
            .get(path.plural())
            .map(|resource| api::TypeMeta {
                api_version: resource.api_version.clone(),
                kind: resource.kind.clone(),
            })
    }
}

impl fmt::Debug for KubeApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KubeApi")
            .field("controllers", &"<BTreeMap<String, Controller>>")
            .finish()
    }
}

fn serialize_to_body<T: serde::Serialize>(data: T) -> kube::Result<Body> {
    json::to_vec(&data)
        .map(Body::from)
        .map_err(kube::Error::SerdeError)
}
