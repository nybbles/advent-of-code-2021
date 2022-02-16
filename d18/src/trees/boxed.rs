pub type SubtreeBox<T> = Box<Tree<T>>

#[derive(Debug)]
pub enum Tree<T> {
  Leaf(T),
  NonLeaf {
    left: SubtreeBox<T>,
    right: SubtreeBox<T>,
  }
}

impl<T: Default> Default for Tree<T> {
  fn default() -> Self {
    Tree::Leaf(T::default())
  }
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