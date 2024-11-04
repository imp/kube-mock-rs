use api::ResourceExt;
use k8s::NamespaceExt;
use k8s::NodeExt;
use k8s::PodExt;

use super::*;

#[derive(Clone, Debug, Default)]
pub struct KubeMockBuilder {
    pub namespaces: HashMap<String, corev1::Namespace>,
    pub nodes: HashMap<String, corev1::Node>,
    pub pods: HashMap<String, corev1::Pod>,
}

impl KubeMockBuilder {
    pub fn new() -> Self {
        let namespaces = ["kube-system", "kube-public", "default"]
            .into_iter()
            .map(corev1::Namespace::new)
            .map(|ns| (ns.name_any(), ns))
            .collect();

        Self {
            namespaces,
            ..Self::default()
        }
    }

    pub fn nodes(self, count: usize) -> Self {
        let nodes = (0..count)
            .map(|ix| format!("node-{ix}"))
            .map(corev1::Node::new)
            .map(|node| (node.name_any(), node))
            .collect();
        Self { nodes, ..self }
    }

    pub fn pod(mut self, name: impl ToString) -> Self {
        let pod = corev1::Pod::new(name);
        self.pods.insert(pod.name_any(), pod);
        self
    }
}
