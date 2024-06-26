use dioxus_core::{DynamicNode::Component, VNode};

/// Extension trait for [`VNode`].
pub trait VNodeExt {
    /// Returns `true` if `self` has a given component node as its child.
    fn has_component(&self, name: &str) -> bool;
}

impl VNodeExt for VNode {
    fn has_component(&self, name: &str) -> bool {
        self.dynamic_nodes.iter().any(|node| {
            if let Component(node) = node {
                node.name == name
            } else {
                false
            }
        })
    }
}
