use super::*;

pub use namespace::Namespaces;
pub use node::Nodes;

mod namespace;
mod node;

pub trait Controller: fmt::Debug + Send + Sync {
    fn type_meta(&self) -> api::TypeMeta;

    // fn route_prefix(&self) -> String;

    fn handle(
        &mut self,
        resource: ParsedResource,
        verb: Verb,
        data: json::Value,
    ) -> kube::Result<json::Value> {
        match verb {
            Verb::Create => self.apply(resource, data),
            Verb::Get => self.get(resource, data),
            Verb::List => todo!(),
            Verb::Watch => todo!(),
            Verb::Delete => todo!(),
            Verb::DeleteCollection => todo!(),
            Verb::Update => todo!(),
            Verb::Patch => todo!(),
        }
    }

    fn apply(&mut self, resource: ParsedResource, data: json::Value) -> kube::Result<json::Value>;
    fn get(&mut self, resource: ParsedResource, data: json::Value) -> kube::Result<json::Value>;
}

fn from_json<K>(object: json::Value) -> kube::Result<K>
where
    K: kube::Resource + serde::de::DeserializeOwned,
{
    json::from_value(object).map_err(kube::Error::SerdeError)
}

fn object_to_json<K>(object: &K) -> kube::Result<json::Value>
where
    K: kube::Resource + serde::Serialize,
{
    json::to_value(object).map_err(kube::Error::SerdeError)
}

fn list_to_json<K>(objects: &[K]) -> kube::Result<json::Value>
where
    K: kube::Resource + serde::Serialize,
{
    json::to_value(objects).map_err(kube::Error::SerdeError)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn send<T: Send>() {}

    #[test]
    fn node_send() {
        send::<Nodes>();
    }

    #[test]
    fn send_kubeapi() {
        send::<KubeApi>();
    }
}
