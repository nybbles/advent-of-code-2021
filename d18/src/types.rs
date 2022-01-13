use std::fmt;
use trees::Tree;

pub type LeafValue = u32;
pub struct NodeValue(pub Option<LeafValue>);
pub type SnailfishNumber = Tree<NodeValue>;

impl fmt::Display for NodeValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.0 {
      Some(value) => write!(f, "{}", value),
      None => write!(f, "{}", "Non-leaf"),
    }
  }
}
