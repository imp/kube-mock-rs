use k8s::NodeExt;

use super::*;

#[derive(Debug, Default)]
pub struct Nodes {
    inventory: BTreeMap<String, corev1::Node>,
}

impl Nodes {
    pub fn example<'a>(names: impl IntoIterator<Item = &'a str>) -> Self {
        let inventory = names
            .into_iter()
            .map(corev1::Node::new)
            .map(|node| (node.name_any(), node))
            .collect();
        Self { inventory }
    }

    pub fn boxed(self) -> Box<dyn Controller> {
        Box::new(self)
    }
}

impl Controller for Nodes {
    fn type_meta(&self) -> api::TypeMeta {
        api::TypeMeta::resource::<corev1::Node>()
    }

    fn apply(
        &mut self,
        resource: ParsedResource,
        object: json::Value,
    ) -> kube::Result<json::Value> {
        tracing::debug!(?resource, %object);
        let node = from_json::<corev1::Node>(object)?;
        let key = node.name_any();
        *self.inventory.entry(key).or_default() = node;
        Ok(json::Value::Null)
    }

    #[tracing::instrument(ret)]
    fn get(&mut self, resource: ParsedResource, data: json::Value) -> kube::Result<json::Value> {
        assert_eq!(data, json::Value::default());
        if let Some(name) = resource.name() {
            object_to_json(&corev1::Node::new(name))
        } else {
            list_to_json(&[corev1::Node::new("node-1"), corev1::Node::new("node-2")])
        }
    }
}
