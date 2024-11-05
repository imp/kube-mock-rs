use super::*;

pub use inventory::ClusterInventory;
pub use inventory::NamespacedInventory;
pub use namespace::Namespaces;
pub use node::Nodes;
pub use pod::Pods;

mod inventory;
mod namespace;
mod node;
mod pod;

pub trait Controller: fmt::Debug + Send + Sync {
    fn type_meta(&self) -> api::TypeMeta;

    fn key(&self, resource: &ParsedResource) -> String;

    fn create_op(
        &mut self,
        object: api::DynamicObject,
        data: json::Value,
    ) -> Result<(json::Value, http::StatusCode), metav1::Status>;

    fn delete_op(
        &mut self,
        object: api::DynamicObject,
        data: json::Value,
    ) -> Result<(json::Value, http::StatusCode), metav1::Status>;

    fn get_op(
        &mut self,
        object: api::DynamicObject,
        data: json::Value,
    ) -> Result<(json::Value, http::StatusCode), metav1::Status>;

    fn list_op(
        &mut self,
        object: api::DynamicObject,
        data: json::Value,
    ) -> Result<(json::Value, http::StatusCode), metav1::Status>;

    fn handle(
        &mut self,
        resource: ParsedResource,
        verb: Verb,
        data: json::Value,
    ) -> Result<json::Value, metav1::Status> {
        match verb {
            Verb::Create => self.create(resource, data),
            Verb::Get => self.get(resource, data),
            Verb::List => self.list(resource, data),
            Verb::Watch => todo!(),
            Verb::Delete => todo!(),
            Verb::DeleteCollection => todo!(),
            Verb::Update => todo!(),
            Verb::Patch => todo!(),
        }
    }

    fn create(
        &mut self,
        resource: ParsedResource,
        data: json::Value,
    ) -> Result<json::Value, metav1::Status>;

    fn get(
        &mut self,
        resource: ParsedResource,
        data: json::Value,
    ) -> Result<json::Value, metav1::Status>;

    fn list(
        &mut self,
        resource: ParsedResource,
        data: json::Value,
    ) -> Result<json::Value, metav1::Status>;

    fn on_create(&self, data: json::Value) -> json::Value {
        data
    }
}

fn object_to_json<K>(object: &K) -> Result<json::Value, metav1::Status>
where
    K: kube::Resource + serde::Serialize,
{
    json::to_value(object).map_err(metav1::Status::bad_request)
}

fn _list_to_json<K>(objects: &[K]) -> Result<json::Value, metav1::Status>
where
    K: kube::Resource + serde::Serialize,
{
    json::to_value(objects).map_err(metav1::Status::bad_request)
}

fn refvec_to_json<K>(objects: Vec<&K>) -> Result<json::Value, metav1::Status>
where
    K: kube::Resource + serde::Serialize,
{
    json::to_value(objects).map_err(metav1::Status::bad_request)
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
