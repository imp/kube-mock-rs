use super::*;

#[derive(Debug)]
pub struct Router {
    router: matchit::Router<Item>,
    api_resources: HashMap<Key, api::ApiResource>,
}

impl Router {
    pub fn new() -> Self {
        let api_resources = api_resources();
        let router = router();

        Self {
            router,
            api_resources,
        }
    }

    pub fn operation(
        &self,
        parts: http::request::Parts,
    ) -> Option<Result<Operation, metav1::Status>> {
        let method = &parts.method;
        let path = parts.uri.path();
        let matched = self.router.at(path).ok()?;
        let key = Key::from_params(&matched.params);
        tracing::debug!(?key);
        let Some(resource) = self.api_resources.get(&key) else {
            tracing::warn!(?key, "Key didn't match any known resource");
            return Some(Err(metav1::Status::no_such_resource()));
        };
        let name = matched.params.get("name").unwrap_or("");
        let ns = matched.params.get("namespace");
        let object = api::DynamicObject::new(name, resource).optionally_within(ns);
        let operation = Operation::new(method, *matched.value, object);

        Some(operation)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Item {
    ClusterKind,
    ClusterObject,
    NamespacedKind,
    NamespacedObject,
}

impl Item {
    pub fn is_object(&self) -> bool {
        match self {
            Self::ClusterKind | Self::NamespacedKind => false,
            Self::ClusterObject | Self::NamespacedObject => true,
        }
    }

    pub fn is_kind(&self) -> bool {
        match self {
            Self::ClusterKind | Self::NamespacedKind => true,
            Self::ClusterObject | Self::NamespacedObject => false,
        }
    }

    pub fn _is_cluster(&self) -> bool {
        match self {
            Self::ClusterKind | Self::ClusterObject => true,
            Self::NamespacedKind | Self::NamespacedObject => false,
        }
    }

    pub fn _is_namespaced(&self) -> bool {
        match self {
            Self::ClusterKind | Self::ClusterObject => false,
            Self::NamespacedKind | Self::NamespacedObject => true,
        }
    }
}

fn router() -> matchit::Router<Item> {
    let routes = [
        ("/api/{version}/{plural}", Item::ClusterKind),
        ("/api/{version}/{plural}/{name}", Item::ClusterObject),
        (
            "/api/{version}/namespaces/{namespace}/{plural}",
            Item::NamespacedKind,
        ),
        (
            "/api/{version}/namespaces/{namespace}/{plural}/{name}",
            Item::NamespacedObject,
        ),
        ("/apis/{group}/{version}/{plural}", Item::ClusterKind),
        (
            "/apis/{group}/{version}/{plural}/{name}",
            Item::ClusterObject,
        ),
        (
            "/apis/{group}/{version}/namespaces/{namespace}/{plural}",
            Item::NamespacedKind,
        ),
        (
            "/apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}",
            Item::NamespacedObject,
        ),
    ];
    let mut router = matchit::Router::new();
    for (route, value) in routes {
        router.insert(route, value).unwrap(); // We want to panic if static route map fails
    }
    router
}

fn api_resources() -> HashMap<Key, api::ApiResource> {
    [
        api::ApiResource::erase::<corev1::Binding>(&()),
        api::ApiResource::erase::<corev1::ConfigMap>(&()),
        api::ApiResource::erase::<corev1::Endpoints>(&()),
        api::ApiResource::erase::<corev1::Event>(&()),
        api::ApiResource::erase::<corev1::LimitRange>(&()),
        api::ApiResource::erase::<corev1::Namespace>(&()),
        api::ApiResource::erase::<corev1::Node>(&()),
        api::ApiResource::erase::<corev1::PersistentVolume>(&()),
        api::ApiResource::erase::<corev1::PersistentVolumeClaim>(&()),
        api::ApiResource::erase::<corev1::Pod>(&()),
        api::ApiResource::erase::<corev1::PodTemplate>(&()),
        api::ApiResource::erase::<corev1::ReplicationController>(&()),
        api::ApiResource::erase::<corev1::ResourceQuota>(&()),
        api::ApiResource::erase::<corev1::Secret>(&()),
        api::ApiResource::erase::<corev1::Service>(&()),
        api::ApiResource::erase::<corev1::ServiceAccount>(&()),
        api::ApiResource::erase::<appsv1::ControllerRevision>(&()),
        api::ApiResource::erase::<appsv1::DaemonSet>(&()),
        api::ApiResource::erase::<appsv1::Deployment>(&()),
        api::ApiResource::erase::<appsv1::ReplicaSet>(&()),
        api::ApiResource::erase::<appsv1::StatefulSet>(&()),
        api::ApiResource::erase::<batchv1::CronJob>(&()),
        api::ApiResource::erase::<batchv1::Job>(&()),
        api::ApiResource::erase::<rbacv1::ClusterRoleBinding>(&()),
        api::ApiResource::erase::<rbacv1::ClusterRole>(&()),
        api::ApiResource::erase::<rbacv1::RoleBinding>(&()),
        api::ApiResource::erase::<rbacv1::Role>(&()),
    ]
    .into_iter()
    .map(Key::with_api_resource)
    .inspect(|(key, ar)| tracing::debug!(?key, ?ar, "Initializing"))
    .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Key {
    api_version: String,
    plural: String,
}

impl Key {
    fn from_params(params: &matchit::Params<'_, '_>) -> Self {
        let group = params
            .get("group")
            .map_or_else(String::new, |group| format!("{group}/"));
        let version = params.get("version").unwrap_or_default();
        let api_version = format!("{group}{version}");
        let plural = params.get("plural").unwrap_or_default().to_string();
        Self {
            api_version,
            plural,
        }
    }

    fn with_api_resource(ar: api::ApiResource) -> (Self, api::ApiResource) {
        let key = Self {
            api_version: ar.api_version.to_string(),
            plural: ar.plural.to_string(),
        };

        (key, ar)
    }
}
