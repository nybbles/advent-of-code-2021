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
enum ZipperDirection {
  Left,
  Right,
}

#[derive(Debug)]
enum Zipper<T> {
  Down {
    parent: Box<Zipper<T>>,
    direction: ZipperDirection,
    focused_subtree: Option<Tree<T>>,
    ignored_subtree: Tree<T>,
  },
  /*
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
  */
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

  fn down(&mut self, direction: ZipperDirection) -> ControlFlow<()> {
    match self {
      Zipper::Tombstone => panic!("Logic error"),
      Zipper::Down {
        focused_subtree, ..
      } => {
        let focused_subtree_val = focused_subtree.take().unwrap();
        match focused_subtree_val {
          Tree::Leaf(_) => {
            focused_subtree.replace(focused_subtree_val);
            ControlFlow::Break(())
          }
          Tree::NonLeaf { left, right } => {
            let (ignored_subtree, focused_subtree) = match direction {
              ZipperDirection::Left => (*right, Some(*left)),
              ZipperDirection::Right => (*left, Some(*right)),
            };
            *self = Zipper::Down {
              parent: Box::new(mem::take(self)),
              ignored_subtree: ignored_subtree,
              focused_subtree: focused_subtree,
              direction: direction,
            };
            ControlFlow::Continue(())
          }
        }
      }

      Zipper::Top { tree } => match tree.take().unwrap() {
        Tree::Leaf(_) => ControlFlow::Break(()),
        Tree::NonLeaf { left, right } => {
          let (ignored_subtree, focused_subtree) = match direction {
            ZipperDirection::Left => (*right, Some(*left)),
            ZipperDirection::Right => (*left, Some(*right)),
          };
          *self = Zipper::Down {
            parent: Box::new(mem::take(self)),
            direction: direction,
            ignored_subtree: ignored_subtree,
            focused_subtree: focused_subtree,
          };
          ControlFlow::Continue(())
        }
      },
    }
  }

  pub fn left(&mut self) -> ControlFlow<()> {
    self.down(ZipperDirection::Left)
  }

  pub fn right(&mut self) -> ControlFlow<()> {
    self.down(ZipperDirection::Right)
  }

  /*
  pub fn up(&self) -> Zipper <T> {}

  pub fn attach(&self, Tree<T>) -> Tree<T> {}

  pub fn to_tree(&self) -> Tree<T> {

  }
  */
}

#[test]
fn test_tree_zipper_left() {
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

#[test]
fn test_tree_zipper_right() {
  use crate::trees::boxed::parser::parse_tree;
  use crate::types::LeafValue;

  let tree = parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap();
  let mut zipper = Zipper::new(tree);

  println!("{:?}", zipper);

  let result = zipper.right();
  assert_eq!(result, ControlFlow::Continue(()));
  println!("{:?}", zipper);

  let result = zipper.right();
  assert_eq!(result, ControlFlow::Continue(()));
  println!("{:?}", zipper);

  let result = zipper.right();
  assert_eq!(result, ControlFlow::Break(()));
  println!("{:?}", zipper);
}
