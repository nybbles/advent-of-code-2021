use std::fmt;

pub type LeafValue = u8;
pub struct NodeValue(pub Option<LeafValue>);

impl fmt::Display for NodeValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.0 {
      Some(value) => write!(f, "{}", value),
      None => write!(f, "{}", "Non-leaf"),
    }
  }
}
