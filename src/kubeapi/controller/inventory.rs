use std::collections::hash_map::Entry;

use k8s_openapi_ext::TimeExt;

use super::*;

#[derive(Debug, Default)]
pub struct ClusterInventory<K> {
    inventory: HashMap<String, K>,
}

impl<K> ClusterInventory<K>
where
    K: kube::Resource<DynamicType = (), Scope = kube::core::ClusterResourceScope>
        + fmt::Debug
        + Send
        + Sync
        + serde::Serialize
        + 'static,
    for<'de> K: serde::Deserialize<'de>,
{
    pub(crate) fn with_inventory(inventory: HashMap<String, K>) -> Self {
        Self { inventory }
    }

    pub(crate) fn boxed(self) -> Box<dyn Controller> {
        Box::new(self)
    }
}

impl<K> Controller for ClusterInventory<K>
where
    K: kube::Resource<DynamicType = (), Scope = kube::core::ClusterResourceScope>
        + fmt::Debug
        + Send
        + Sync
        + serde::Serialize,
    for<'de> K: serde::Deserialize<'de>,
{
    fn type_meta(&self) -> api::TypeMeta {
        api::TypeMeta::resource::<K>()
    }

    fn key(&self, resource: &ParsedResource) -> String {
        if let Some(name) = resource.name() {
            if let Some(namespace) = resource._namespace() {
                format!("{namespace}/{name}")
            } else {
                name.to_string()
            }
        } else {
            String::new()
        }
    }

    fn create(
        &mut self,
        resource: ParsedResource,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, metav1::Status> {
        tracing::debug!(?resource, %data);
        let data = self.on_create(data);
        let value = from_json::<K>(data)?;
        let name = value.name_any();
        match self.inventory.entry(name) {
            Entry::Occupied(entry) => Err(metav1::Status::already_exists::<K>(entry.key())),
            Entry::Vacant(entry) => {
                let object = entry.insert(value);
                let meta = object.meta_mut();
                meta.creation_timestamp = Some(metav1::Time::now());
                meta.namespace.get_or_insert_with(|| "default".to_string());
                meta.resource_version = Some("1".to_string());
                meta.uid = Some(uuid::Uuid::new_v4().to_string());
                object_to_json(object)
            }
        }
    }

    fn get(
        &mut self,
        resource: ParsedResource,
        data: json::Value,
    ) -> Result<json::Value, metav1::Status> {
        tracing::debug!(?resource, %data);
        assert_eq!(data, json::Value::default());
        let key = self.key(&resource);
        self.inventory
            .get(&key)
            .ok_or_else(|| metav1::Status::not_found::<K>(key))
            .and_then(object_to_json)
    }

    fn list(
        &mut self,
        resource: ParsedResource,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, metav1::Status> {
        tracing::debug!(?resource, %data);
        Err(metav1::Status::method_not_allowed())
    }
}

#[derive(Debug, Default)]
pub struct NamespacedInventory<K> {
    inventory: HashMap<String, K>,
}

impl<K> NamespacedInventory<K>
where
    K: kube::Resource<DynamicType = (), Scope = kube::core::NamespaceResourceScope>
        + fmt::Debug
        + Send
        + Sync
        + serde::Serialize
        + 'static,
    for<'de> K: serde::Deserialize<'de>,
{
    pub(crate) fn with_inventory(inventory: HashMap<String, K>) -> Self {
        Self { inventory }
    }

    pub(crate) fn boxed(self) -> Box<dyn Controller> {
        Box::new(self)
    }
}

impl<K> Controller for NamespacedInventory<K>
where
    K: kube::Resource<DynamicType = (), Scope = kube::core::NamespaceResourceScope>
        + fmt::Debug
        + Send
        + Sync
        + serde::Serialize,
    for<'de> K: serde::Deserialize<'de>,
{
    fn type_meta(&self) -> api::TypeMeta {
        api::TypeMeta::resource::<K>()
    }

    fn key(&self, resource: &ParsedResource) -> String {
        if let Some(name) = resource.name() {
            if let Some(namespace) = resource._namespace() {
                format!("{namespace}/{name}")
            } else {
                name.to_string()
            }
        } else {
            String::new()
        }
    }

    fn create(
        &mut self,
        resource: ParsedResource,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, metav1::Status> {
        tracing::debug!(?resource, %data);
        let value = from_json::<K>(data)?;
        let name = value.name_any();
        match self.inventory.entry(name) {
            Entry::Occupied(entry) => Err(metav1::Status::already_exists::<K>(entry.key())),
            Entry::Vacant(entry) => {
                let object = entry.insert(value);
                let meta = object.meta_mut();
                meta.creation_timestamp = Some(metav1::Time::now());
                meta.namespace.get_or_insert_with(|| "default".to_string());
                meta.resource_version = Some("1".to_string());
                meta.uid = Some(uuid::Uuid::new_v4().to_string());
                object_to_json(object)
            }
        }
    }

    fn get(
        &mut self,
        resource: ParsedResource,
        data: json::Value,
    ) -> Result<json::Value, metav1::Status> {
        tracing::debug!(?resource, %data);
        assert_eq!(data, json::Value::default());
        if let Some(name) = resource.name() {
            self.inventory
                .get(name)
                .ok_or_else(|| metav1::Status::not_found::<K>(name))
                .and_then(object_to_json)
        } else {
            let objects = self.inventory.values().collect();
            refvec_to_json::<K>(objects)
        }
    }

    fn list(
        &mut self,
        resource: ParsedResource,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, metav1::Status> {
        tracing::debug!(?resource, %data);
        Err(metav1::Status::method_not_allowed())
    }
}
