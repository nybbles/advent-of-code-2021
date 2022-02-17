use crate::trees::boxed::Tree;
use std::mem;
use std::ops::ControlFlow;

// based on:
// 1. https://www.reddit.com/r/rust/comments/jkh99u/comment/gaj1xse/
// 2. http://learnyouahaskell.com/zippers
// subtree (for Left and Right) and tree (for Top) are Options because they are
// set to None when the Zipper is moved down. The next Zipper takes ownership of
// the subtree it is focusing on.

#[derive(Debug)]
enum Zipper<T> {
  Left {
    subtree: Option<Tree<T>>,
    parent: Box<Zipper<T>>,
    right: Tree<T>,
  },
  Right {
    subtree: Option<Tree<T>>,
    parent: Box<Zipper<T>>,
    left: Tree<T>,
  },
  Top {
    tree: Option<Tree<T>>,
  },
  Tombstone,
}

impl<T: Default> Default for Zipper<T> {
  fn default() -> Self {
    Zipper::Tombstone
  }
}

impl<T: Default> Zipper<T> {
  pub fn new(tree: Tree<T>) -> Zipper<T> {
    Zipper::Top { tree: Some(tree) }
  }

  pub fn left(&mut self) -> ControlFlow<()> {
    match self {
      Zipper::Tombstone => panic!("Logic error"),
      Zipper::Left { subtree, .. } | Zipper::Right { subtree, .. } => {
        let subtree_val = subtree.take().unwrap();
        match subtree_val {
          Tree::Leaf(_) => {
            subtree.replace(subtree_val);
            ControlFlow::Break(())
          }
          Tree::NonLeaf { left, right } => {
            *self = Zipper::Left {
              parent: Box::new(mem::take(self)),
              right: *right,
              subtree: Some(*left),
            };
            ControlFlow::Continue(())
          }
        }
      }

      Zipper::Top { tree } => match tree.take().unwrap() {
        Tree::Leaf(_) => ControlFlow::Break(()),
        Tree::NonLeaf { left, right } => {
          *self = Zipper::Left {
            parent: Box::new(mem::take(self)),
            right: *right,
            subtree: Some(*left),
          };
          ControlFlow::Continue(())
        }
      },
    }
  }

  pub fn right(&mut self) -> ControlFlow<()> {}

  /*
  pub fn up(&self) -> Zipper <T> {}

  pub fn attach(&self, Tree<T>) -> Tree<T> {}

  pub fn to_tree(&self) -> Tree<T> {

  }
  */
}

#[test]
fn test_tree_zipper() {
  use crate::trees::boxed::parser::parse_tree;
  use crate::types::LeafValue;

  let tree = parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap();
  let mut zipper = Zipper::new(tree);

  println!("{:?}", zipper);

  let result = zipper.left();
  assert_eq!(result, ControlFlow::Continue(()));
  println!("{:?}", zipper);

  let result = zipper.left();
  assert_eq!(result, ControlFlow::Continue(()));
  println!("{:?}", zipper);

  let result = zipper.left();
  assert_eq!(result, ControlFlow::Break(()));
  println!("{:?}", zipper);
}
