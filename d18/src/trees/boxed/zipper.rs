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

  pub fn focused_subtree(&self) -> &Tree<T> {
    let treeopt = match self {
      Zipper::Tombstone => panic!("Logic error"),
      Zipper::Top { ref tree } => tree,
      Zipper::Down {
        ref focused_subtree,
        ..
      } => focused_subtree,
    };

    match treeopt {
      Some(ref tree) => tree,
      None => panic!("Logic error"),
    }
  }

  pub fn down(&mut self, direction: ZipperDirection) -> ControlFlow<()> {
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

  pub fn up(&mut self) -> ControlFlow<()> {
    match self {
      Zipper::Tombstone => panic!("Logic error"),
      Zipper::Top { .. } => ControlFlow::Break(()),
      Zipper::Down {
        parent,
        direction,
        focused_subtree,
        ignored_subtree,
      } => {
        // need to build back the tree that was here
        // build a new tree by putting focused subtree on correct side based on
        // direction, and put ignored subtree on the other side

        let new_focused_subtree = {
          let (left, right) = match direction {
            ZipperDirection::Left => (focused_subtree.take().unwrap(), mem::take(ignored_subtree)),
            ZipperDirection::Right => (mem::take(ignored_subtree), focused_subtree.take().unwrap()),
          };
          Tree::NonLeaf {
            // directon needs to go here
            left: Box::new(left),
            right: Box::new(right),
          }
        };

        let zipper = mem::take(parent);
        match *zipper {
          Zipper::Tombstone => panic!("Logic error"),
          Zipper::Top { .. } => {
            *self = Zipper::Top {
              tree: Some(new_focused_subtree),
            };
            ControlFlow::Continue(())
          }
          Zipper::Down {
            direction,
            parent,
            ignored_subtree,
            ..
          } => {
            //
            *self = Zipper::Down {
              direction: direction,
              parent: parent,
              focused_subtree: Some(new_focused_subtree),
              ignored_subtree: ignored_subtree,
            };
            ControlFlow::Continue(())
          }
        }
      }
    }
  }

  /*
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

  println!("{:?}", zipper.focused_subtree());

  let result = zipper.left();
  assert_eq!(result, ControlFlow::Continue(()));
  println!("{:?}", zipper.focused_subtree());

  let result = zipper.left();
  assert_eq!(result, ControlFlow::Continue(()));
  println!("{:?}", zipper.focused_subtree());

  let result = zipper.left();
  assert_eq!(result, ControlFlow::Break(()));
  println!("{:?}", zipper.focused_subtree());

  let result = zipper.up();
  assert_eq!(result, ControlFlow::Continue(()));
  println!("{:?}", zipper.focused_subtree());

  let result = zipper.up();
  assert_eq!(result, ControlFlow::Continue(()));
  println!("{:?}", zipper.focused_subtree());

  let result = zipper.up();
  assert_eq!(result, ControlFlow::Break(()));
  println!("{:?}", zipper.focused_subtree());
}

#[test]
fn test_tree_zipper_right() {
  use crate::trees::boxed::parser::parse_tree;
  use crate::types::LeafValue;

  let tree = parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap();
  let mut zipper = Zipper::new(tree);

  println!("{:?}", zipper.focused_subtree());

  let result = zipper.right();
  assert_eq!(result, ControlFlow::Continue(()));
  println!("{:?}", zipper.focused_subtree());

  let result = zipper.right();
  assert_eq!(result, ControlFlow::Continue(()));
  println!("{:?}", zipper.focused_subtree());

  let result = zipper.right();
  assert_eq!(result, ControlFlow::Break(()));
  println!("{:?}", zipper.focused_subtree());
}
