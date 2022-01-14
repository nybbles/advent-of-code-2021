use std::fmt;
use trees::{Node, Tree};

pub type LeafValue = u32;
pub type NodeValue = Option<LeafValue>;
pub type SnailfishNumber = Tree<NodeValue>;

#[cfg(test)]
pub fn trees_eq(root0: &Node<NodeValue>, root1: &Node<NodeValue>) -> bool {
  if root0.node_count() != root1.node_count() {
    return false;
  }

  if root0.data() != root1.data() {
    return false;
  }

  for (subtree0, subtree1) in root0.iter().zip(root1.iter()) {
    if !trees_eq(subtree0, subtree1) {
      return false;
    }
  }

  return true;
}
