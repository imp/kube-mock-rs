use super::*;

#[derive(Clone, Debug, Default)]
pub struct KubeMockBuilder {
    nodes: BTreeMap<String, corev1::Node>,
}

impl KubeMockBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn nodes(self, count: usize) -> Self {
        let nodes = (0..count)
            .map(|ix| format!("node-{ix}"))
            .map(|name| (name, corev1::Node::default()))
            .collect();
        Self { nodes }
    }
}
