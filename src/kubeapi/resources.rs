use super::*;

pub(super) fn api_resources() -> HashMap<String, api::ApiResource> {
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
    .map(|resource| (resource.plural.clone(), resource))
    .collect()
}
