// use k8s::NodeExt;

use super::*;

pub type Nodes = ClusterInventory<corev1::Node>;

// #[derive(Debug, Default)]
// pub struct Nodes {
//     inventory: HashMap<String, corev1::Node>,
// }

// impl Nodes {
//     pub(crate) fn from_inventory(inventory: HashMap<String, corev1::Node>) -> Self {
//         Self { inventory }
//     }

//     pub fn boxed(self) -> Box<dyn Controller> {
//         Box::new(self)
//     }
// }

// impl Controller for Nodes {
//     fn type_meta(&self) -> api::TypeMeta {
//         api::TypeMeta::resource::<corev1::Node>()
//     }

//     fn key(&self, resource: &ParsedResource) -> String {
//         resource.name().unwrap_or_default().to_string()
//     }

//     fn create(
//         &mut self,
//         resource: ParsedResource,
//         object: json::Value,
//     ) -> Result<json::Value, metav1::Status> {
//         tracing::debug!(?resource, %object);
//         let node = from_json::<corev1::Node>(object)?;
//         let key = node.name_any();
//         *self.inventory.entry(key).or_default() = node;
//         Ok(json::Value::Null)
//     }

//     #[tracing::instrument(ret)]
//     fn get(
//         &mut self,
//         resource: ParsedResource,
//         data: json::Value,
//     ) -> Result<json::Value, metav1::Status> {
//         assert_eq!(data, json::Value::default());
//         if let Some(name) = resource.name() {
//             self.inventory
//                 .get(name)
//                 .ok_or_else(|| metav1::Status::not_found::<corev1::Node>(name))
//                 .and_then(object_to_json)
//         } else {
//             list_to_json(&[corev1::Node::new("node-1"), corev1::Node::new("node-2")])
//         }
//     }

//     fn list(
//         &mut self,
//         resource: ParsedResource,
//         data: serde_json::Value,
//     ) -> Result<serde_json::Value, metav1::Status> {
//         tracing::debug!(?resource, %data);
//         Err(metav1::Status::method_not_allowed())
//     }
// }
