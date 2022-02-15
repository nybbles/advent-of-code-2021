// https://discord.com/channels/442252698964721669/443150878111694848/943216605066846268

// How to handle leaf nodes?
// Where to store value?
#[derive(Debug)]
pub enum Tree<T: PartialEq> {
  Leaf(T),
  NonLeaf {
    left: Box<Tree<T>>,
    right: Box<Tree<T>>,
  },
}

impl<T: PartialEq> PartialEq for Tree<T> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Tree::Leaf(_), Tree::NonLeaf { .. }) | (Tree::NonLeaf { .. }, Tree::Leaf(_)) => false,
      (Tree::Leaf(value0), Tree::Leaf(value1)) => value0 == value1,
      (
        Tree::NonLeaf {
          left: self_left,
          right: self_right,
        },
        Tree::NonLeaf {
          left: other_left,
          right: other_right,
        },
      ) => self_left == other_left && self_right == other_right,
    }
  }
}
