use std::collections::hash_map::Entry;

use api::Resource;
use k8s::TimeExt;

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
        + for<'de> serde::Deserialize<'de>
        + 'static,
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
        + serde::Serialize
        + for<'de> serde::Deserialize<'de>,
{
    fn type_meta(&self) -> api::TypeMeta {
        api::TypeMeta::resource::<K>()
    }

    fn key_op(&self, object: &api::DynamicObject) -> String {
        object.name_any()
    }

    // fn list(
    //     &mut self,
    //     resource: ParsedResource,
    //     data: serde_json::Value,
    // ) -> Result<serde_json::Value, metav1::Status> {
    //     tracing::debug!(?resource, %data);
    //     Err(metav1::Status::method_not_allowed())
    // }

    fn create(
        &mut self,
        object: api::DynamicObject,
        data: serde_json::Value,
    ) -> Result<(serde_json::Value, http::StatusCode), metav1::Status> {
        tracing::debug!(?object, %data, "create");
        let key = self.key_op(&object);
        let k = from_json::<K>(data)?;
        match self.inventory.entry(key) {
            Entry::Occupied(entry) => Err(metav1::Status::already_exists::<K>(entry.key())),
            Entry::Vacant(entry) => {
                let object = entry.insert(k);
                init_meta(object.meta_mut());
                object_to_json(object).map(|value| (value, http::StatusCode::CREATED))
            }
        }
    }

    fn delete(
        &mut self,
        object: api::DynamicObject,
        data: serde_json::Value,
    ) -> Result<(serde_json::Value, http::StatusCode), metav1::Status> {
        tracing::debug!(?object, %data, "delete");
        todo!("delete")
    }

    fn get(
        &mut self,
        object: api::DynamicObject,
        data: serde_json::Value,
    ) -> Result<(serde_json::Value, http::StatusCode), metav1::Status> {
        tracing::debug!(?object, %data, "get");
        assert_eq!(data, json::Value::default());
        let key = self.key_op(&object);
        self.inventory
            .get(&key)
            .ok_or_else(|| metav1::Status::not_found::<K>(key))
            .and_then(object_to_json)
            .map(|value| (value, http::StatusCode::OK))
    }

    fn list(
        &mut self,
        object: api::DynamicObject,
        data: serde_json::Value,
    ) -> Result<(serde_json::Value, http::StatusCode), metav1::Status> {
        tracing::debug!(?object, %data, "list");
        todo!("list")
    }

    fn update(
        &mut self,
        object: api::DynamicObject,
        data: serde_json::Value,
    ) -> Result<(serde_json::Value, http::StatusCode), metav1::Status> {
        tracing::debug!(?object, %data, "update");
        todo!("update")
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
        + for<'de> serde::Deserialize<'de>
        + 'static,
{
    fn _lookup(&self, object: &api::DynamicObject) -> Option<&K> {
        let name = object.name_any();
        if let Some(namespace) = object.namespace() {
            let key = format!("{namespace}/{name}");
            self.inventory.get(&key)
        } else {
            self.inventory.values().find(|k| k.name_any() == name)
        }
    }
}

impl<K> NamespacedInventory<K>
where
    K: kube::Resource<DynamicType = (), Scope = kube::core::NamespaceResourceScope>
        + fmt::Debug
        + Send
        + Sync
        + serde::Serialize
        + for<'de> serde::Deserialize<'de>
        + 'static,
{
    pub(crate) fn with_inventory(inventory: HashMap<String, K>) -> Self {
        Self { inventory }
    }

    pub(crate) fn boxed(self) -> Box<dyn Controller> {
        Box::new(self)
    }

    fn default_namespace() -> String {
        String::from("default")
    }
}

impl<K> Controller for NamespacedInventory<K>
where
    K: kube::Resource<DynamicType = (), Scope = kube::core::NamespaceResourceScope>
        + fmt::Debug
        + Send
        + Sync
        + serde::Serialize
        + for<'de> serde::Deserialize<'de>
        + 'static,
{
    fn type_meta(&self) -> api::TypeMeta {
        api::TypeMeta::resource::<K>()
    }

    fn key_op(&self, object: &api::DynamicObject) -> String {
        let name = object.name_any();
        if let Some(namespace) = object.namespace() {
            format!("{namespace}/{name}")
        } else {
            format!("default/{name}")
        }
    }

    // fn get(
    //     &mut self,
    //     resource: ParsedResource,
    //     data: json::Value,
    // ) -> Result<json::Value, metav1::Status> {
    //     tracing::debug!(?resource, %data);
    //     assert_eq!(data, json::Value::default());
    //     if let Some(name) = resource.name() {
    //         self.inventory
    //             .get(name)
    //             .ok_or_else(|| metav1::Status::not_found::<K>(name))
    //             .and_then(object_to_json)
    //     } else {
    //         let objects = self.inventory.values().collect();
    //         refvec_to_json::<K>(objects)
    //     }
    // }

    // fn list(
    //     &mut self,
    //     resource: ParsedResource,
    //     data: serde_json::Value,
    // ) -> Result<serde_json::Value, metav1::Status> {
    //     tracing::debug!(?resource, %data);
    //     Err(metav1::Status::method_not_allowed())
    // }

    fn create(
        &mut self,
        mut object: api::DynamicObject,
        data: serde_json::Value,
    ) -> Result<(serde_json::Value, http::StatusCode), metav1::Status> {
        tracing::debug!(?object, %data, "create");
        let namespace = object
            .meta_mut()
            .namespace
            .get_or_insert_with(Self::default_namespace)
            .clone();
        let name = object.name_any();
        let key = format!("{namespace}/{name}");
        let k = from_json::<K>(data)?;
        k.namespace().get_or_insert(namespace);
        if object.namespace() != k.namespace() {
            Err(metav1::Status::failure("conflict"))?;
        }
        match self.inventory.entry(key) {
            Entry::Occupied(entry) => Err(metav1::Status::already_exists::<K>(entry.key())),
            Entry::Vacant(entry) => {
                let object = entry.insert(k);
                init_meta(object.meta_mut());
                object_to_json(object).map(|value| (value, http::StatusCode::CREATED))
            }
        }
    }

    fn delete(
        &mut self,
        object: api::DynamicObject,
        data: serde_json::Value,
    ) -> Result<(serde_json::Value, http::StatusCode), metav1::Status> {
        tracing::debug!(?object, %data, "delete");
        todo!("delete")
    }

    fn get(
        &mut self,
        object: api::DynamicObject,
        data: serde_json::Value,
    ) -> Result<(serde_json::Value, http::StatusCode), metav1::Status> {
        tracing::debug!(?object, %data, "get");
        assert_eq!(data, json::Value::default());
        let key = self.key_op(&object);

        self.inventory
            .get(&key)
            .ok_or_else(|| metav1::Status::not_found::<K>(key))
            .and_then(object_to_json)
            .map(|value| (value, http::StatusCode::OK))
    }

    fn list(
        &mut self,
        object: api::DynamicObject,
        data: serde_json::Value,
    ) -> Result<(serde_json::Value, http::StatusCode), metav1::Status> {
        tracing::debug!(?object, %data, "list");
        todo!("list")
    }

    fn update(
        &mut self,
        object: api::DynamicObject,
        data: serde_json::Value,
    ) -> Result<(serde_json::Value, http::StatusCode), metav1::Status> {
        tracing::debug!(?object, %data, "update");
        todo!("update")
    }
}

fn init_meta(meta: &mut metav1::ObjectMeta) {
    meta.creation_timestamp = Some(metav1::Time::now());
    meta.resource_version = Some("1".to_string());
    meta.uid = Some(uuid::Uuid::new_v4().to_string());
}
