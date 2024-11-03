use k8s::Resource as _;
use kube::Resource as _;
use kube::ResourceExt;

use super::*;

use controller::Controller;
use parser::Error;
use parser::ParsedResource;
use verbs::Verb;

mod controller;
mod error;
mod parser;
mod resources;
mod verbs;

// #[derive(Debug)]
pub struct KubeApi {
    resources: HashMap<String, api::ApiResource>,

    // nodes: BTreeMap<String, corev1::Node>,
    controllers: HashMap<api::TypeMeta, Box<dyn Controller>>,
}

impl KubeApi {
    pub fn new() -> Self {
        let resources = resources::api_resources();

        let nodes = controller::Nodes::example(["node-1", "node-2"]).boxed();
        let namespaces = controller::Namespaces::example().boxed();

        let controllers = [nodes, namespaces]
            .into_iter()
            .map(|controller| (controller.type_meta(), controller))
            .collect();

        Self {
            resources,
            controllers,
        }
    }

    pub fn dispatch(
        &mut self,
        resource: ParsedResource,
        verb: Verb,
        data: json::Value,
    ) -> kube::Result<json::Value> {
        let meta = self
            .get_type_meta(&resource)
            .ok_or(kube::Error::LinesCodecMaxLineLengthExceeded)?;

        tracing::debug!(?meta);
        self.controllers
            .get_mut(&meta)
            .inspect(|controller| tracing::debug!(?controller))
            .ok_or(kube::Error::LinesCodecMaxLineLengthExceeded)?
            .handle(resource, verb, data)
    }

    pub async fn parse_request(
        &self,
        request: Request<Body>,
    ) -> kube::Result<(ParsedResource, Verb, json::Value)> {
        let _method = request.method();
        let verb = Verb::from_request(&request);
        let uri = request.uri();
        let parsed_path = uri.path().parse()?;
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
