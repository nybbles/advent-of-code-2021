// this one has an example of a mutable iterator:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=985c685d5809121fc93c3bdeb64fa755

pub mod iter;

use std::cell::RefCell;
use std::rc::Rc;

use iter::TreeIter;

pub type SubtreeRef<T> = Rc<RefCell<Tree<T>>>;

#[derive(Debug)]
pub enum Tree<T> {
  Leaf(T),
  NonLeaf {
    left: SubtreeRef<T>,
    right: SubtreeRef<T>,
  },
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

impl<T> Tree<T> {
  pub fn new_non_leaf(left: Tree<T>, right: Tree<T>) -> Tree<T> {
    Tree::NonLeaf {
      left: Rc::new(RefCell::new(left)),
      right: Rc::new(RefCell::new(right)),
    }
  }

  // &self needs to be owned by a Rc<RefCell> and then that Rc needs to be
  // passed in.
  fn iter(root: SubtreeRef<T>) -> TreeIter<T> {
    TreeIter {
      curr_depth: 0,
      next_depth: 0,
      next_visit: vec![root],
      parent: None,
    }
  }
}
