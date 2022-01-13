use std::fmt;
use trees::Tree;

pub type LeafValue = u32;
pub type NodeValue = Option<LeafValue>;
pub type SnailfishNumber = Tree<NodeValue>;
