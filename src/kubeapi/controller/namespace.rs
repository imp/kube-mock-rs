// use k8s::NamespaceExt;

use super::*;

pub type Namespaces = ClusterInventory<corev1::Namespace>;

// #[derive(Debug, Default)]
// pub struct Namespaces {
//     inventory: HashMap<String, corev1::Namespace>,
// }

// impl Namespaces {
//     pub(crate) fn from_inventory(inventory: HashMap<String, corev1::Namespace>) -> Self {
//         Self { inventory }
//     }

//     pub fn boxed(self) -> Box<dyn Controller> {
//         Box::new(self)
//     }
// }

// impl Controller for Namespaces {
//     fn type_meta(&self) -> api::TypeMeta {
//         api::TypeMeta::resource::<corev1::Namespace>()
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
//         let object = from_json::<corev1::Namespace>(object)?;
//         let key = object.name_any();
//         *self.inventory.entry(key).or_default() = object;
//         Ok(json::Value::Null)
//     }

//     fn get(
//         &mut self,
//         resource: ParsedResource,
//         data: json::Value,
//     ) -> Result<json::Value, metav1::Status> {
//         assert_eq!(data, json::Value::default());
//         let key = self.key(&resource);
//         object_to_json(&corev1::Namespace::new(&key))

//         // if let Some(name) = resource.name() {
//         //     object_to_json(&corev1::Namespace::new(name))
//         // } else {
//         //     list_to_json(&[
//         //         corev1::Namespace::new("default"),
//         //         corev1::Namespace::new("kube-system"),
//         //         corev1::Namespace::new("kube-public"),
//         //     ])
//         // }
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
