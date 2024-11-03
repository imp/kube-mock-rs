use k8s::NamespaceExt;

use super::*;

#[derive(Debug, Default)]
pub struct Namespaces {
    inventory: BTreeMap<String, corev1::Namespace>,
}

impl Namespaces {
    pub fn example() -> Self {
        let inventory = [
            corev1::Namespace::new("kube-system"),
            corev1::Namespace::new("kube-public"),
            corev1::Namespace::new("default"),
        ]
        .into_iter()
        .map(|ns| (ns.name_any(), ns))
        .collect();

        Self { inventory }
    }

    pub fn boxed(self) -> Box<dyn Controller> {
        Box::new(self)
    }
}

impl Controller for Namespaces {
    fn type_meta(&self) -> api::TypeMeta {
        api::TypeMeta::resource::<corev1::Namespace>()
    }

    fn apply(
        &mut self,
        resource: ParsedResource,
        object: json::Value,
    ) -> kube::Result<json::Value> {
        tracing::debug!(?resource, %object);
        let object = from_json::<corev1::Namespace>(object)?;
        let key = object.name_any();
        *self.inventory.entry(key).or_default() = object;
        Ok(json::Value::Null)
    }

    fn get(&mut self, resource: ParsedResource, data: json::Value) -> kube::Result<json::Value> {
        assert_eq!(data, json::Value::default());
        if let Some(name) = resource.name() {
            object_to_json(&corev1::Namespace::new(name))
        } else {
            list_to_json(&[
                corev1::Namespace::new("default"),
                corev1::Namespace::new("kube-system"),
                corev1::Namespace::new("kube-public"),
            ])
        }
    }
}
