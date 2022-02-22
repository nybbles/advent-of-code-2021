use crate::trees::boxed::Tree;
use std::mem;
use std::ops::ControlFlow;

// based on:
// 1. https://www.reddit.com/r/rust/comments/jkh99u/comment/gaj1xse/
// 2. http://learnyouahaskell.com/zippers
// subtree (for Left and Right) and tree (for Top) are Options because they are
// set to None when the Zipper is moved down. The next Zipper takes ownership of
// the subtree it is focusing on.

#[derive(Debug, PartialEq, Copy, Clone)]
enum ZipperDirection {
  Left,
  Right,
}

#[derive(Debug, PartialEq)]
pub enum Zipper<T> {
  Down {
    depth: usize,
    parent: Box<Zipper<T>>,
    direction: ZipperDirection,
    focused_subtree: Option<Tree<T>>,
    ignored_subtree: Tree<T>,
  },
  Top {
    tree: Option<Tree<T>>,
  },
  Tombstone,
  Emptied,
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

  pub fn get_depth(&self) -> usize {
    match self {
      Zipper::Tombstone | Zipper::Emptied => panic!("Logic error"),
      Zipper::Top { .. } => 0,
      Zipper::Down { depth, .. } => *depth,
    }
  }

  pub fn focused_subtree(&self) -> &Tree<T> {
    let treeopt = match self {
      Zipper::Tombstone | Zipper::Emptied => panic!("Logic error"),
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
      Zipper::Tombstone | Zipper::Emptied => panic!("Logic error"),
      Zipper::Down {
        depth,
        focused_subtree,
        ..
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
              depth: *depth + 1,
              parent: Box::new(mem::take(self)),
              ignored_subtree: ignored_subtree,
              focused_subtree: focused_subtree,
              direction: direction,
            };
            ControlFlow::Continue(())
          }
        }
      }
      Zipper::Top { tree } => {
        let tree_val = tree.take().unwrap();
        match tree_val {
          Tree::Leaf(_) => {
            tree.replace(tree_val);
            ControlFlow::Break(())
          }
          Tree::NonLeaf { left, right } => {
            let (ignored_subtree, focused_subtree) = match direction {
              ZipperDirection::Left => (*right, Some(*left)),
              ZipperDirection::Right => (*left, Some(*right)),
            };
            *self = Zipper::Down {
              depth: 1,
              parent: Box::new(mem::take(self)),
              direction: direction,
              ignored_subtree: ignored_subtree,
              focused_subtree: focused_subtree,
            };
            ControlFlow::Continue(())
          }
        }
      }
    }
  }

  pub fn left(&mut self) -> ControlFlow<()> {
    self.down(ZipperDirection::Left)
  }

  pub fn right(&mut self) -> ControlFlow<()> {
    self.down(ZipperDirection::Right)
  }

  pub fn up(&mut self) -> ControlFlow<(), ZipperDirection> {
    match self {
      Zipper::Tombstone | Zipper::Emptied => panic!("Logic error"),
      Zipper::Top { .. } => ControlFlow::Break(()),
      Zipper::Down {
        depth,
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
            left: Box::new(left),
            right: Box::new(right),
          }
        };

        let child_direction = *direction;

        let parent_zipper = mem::take(parent);
        match *parent_zipper {
          Zipper::Tombstone | Zipper::Emptied => panic!("Logic error"),
          Zipper::Top { .. } => {
            *self = Zipper::Top {
              tree: Some(new_focused_subtree),
            };
            ControlFlow::Continue(child_direction)
          }
          Zipper::Down {
            direction: parent_direction,
            parent,
            ignored_subtree,
            ..
          } => {
            *self = Zipper::Down {
              depth: *depth - 1,
              direction: parent_direction,
              parent: parent,
              focused_subtree: Some(new_focused_subtree),
              ignored_subtree: ignored_subtree,
            };
            ControlFlow::Continue(child_direction)
          }
        }
      }
    }
  }

  // returns the tree being replaced
  pub fn attach(&mut self, new_tree: Tree<T>) -> Tree<T> {
    match self {
      Zipper::Tombstone | Zipper::Emptied => panic!("Logic error"),
      Zipper::Top { tree }
      | Zipper::Down {
        focused_subtree: tree,
        ..
      } => {
        let old_tree = tree.replace(new_tree);
        old_tree.unwrap()
      }
    }
  }

  pub fn to_tree(&mut self) -> Tree<T> {
    // gives ownership of the tree back to the caller
    while self.up() != ControlFlow::Break(()) {}

    let tree = match self {
      Zipper::Tombstone | Zipper::Emptied | Zipper::Down { .. } => panic!("Logic error"),
      Zipper::Top { tree } => tree.take().unwrap(),
    };

    *self = Zipper::Emptied;

    tree
  }
}

enum ZipperDFSTraversalDirection {
  Up,
  Left,
  Right,
}

#[derive(PartialEq)]
enum ZipperDFSTraversalIterDirection {
  Forward,
  Backward,
}

pub struct ZipperDFSTraversal<T> {
  next_direction: ZipperDFSTraversalDirection,
  iter_direction: ZipperDFSTraversalIterDirection,
  zipper: Zipper<T>,
}

impl<T: Default> ZipperDFSTraversal<T> {
  pub fn new(zipper: Zipper<T>) -> ZipperDFSTraversal<T> {
    ZipperDFSTraversal {
      next_direction: ZipperDFSTraversalDirection::Left,
      iter_direction: ZipperDFSTraversalIterDirection::Forward,
      zipper: zipper,
    }
  }

  pub fn next(&mut self) -> ControlFlow<()> {
    if self.iter_direction != ZipperDFSTraversalIterDirection::Forward {
      self.next_direction = match self.next_direction {
        ZipperDFSTraversalDirection::Right => ZipperDFSTraversalDirection::Up,
        ZipperDFSTraversalDirection::Left => ZipperDFSTraversalDirection::Right,
        ZipperDFSTraversalDirection::Up => ZipperDFSTraversalDirection::Left,
      };
      self.iter_direction = ZipperDFSTraversalIterDirection::Forward;
    }

    match self.next_direction {
      ZipperDFSTraversalDirection::Left => match self.zipper.left() {
        ControlFlow::Break(()) => {
          self.next_direction = ZipperDFSTraversalDirection::Up;
          self.next()
        }
        ControlFlow::Continue(()) => ControlFlow::Continue(()),
      },
      ZipperDFSTraversalDirection::Right => match self.zipper.right() {
        ControlFlow::Break(()) => {
          self.next_direction = ZipperDFSTraversalDirection::Up;
          self.next()
        }
        ControlFlow::Continue(()) => {
          self.next_direction = ZipperDFSTraversalDirection::Left;
          ControlFlow::Continue(())
        }
      },
      ZipperDFSTraversalDirection::Up => match self.zipper.up() {
        ControlFlow::Break(()) => {
          self.next_direction = ZipperDFSTraversalDirection::Left;
          ControlFlow::Break(())
        }
        ControlFlow::Continue(direction) => {
          match direction {
            ZipperDirection::Left => {
              self.next_direction = ZipperDFSTraversalDirection::Right;
            }
            ZipperDirection::Right => {
              self.next_direction = ZipperDFSTraversalDirection::Up;
            }
          }
          ControlFlow::Continue(())
        }
      },
    }
  }

  // What happens when next and prev are called on the same DFS traversal? Does
  // next_direction make sense?
  pub fn prev(&mut self) -> ControlFlow<()> {
    if self.iter_direction != ZipperDFSTraversalIterDirection::Backward {
      self.next_direction = match self.next_direction {
        ZipperDFSTraversalDirection::Left => ZipperDFSTraversalDirection::Up,
        ZipperDFSTraversalDirection::Right => ZipperDFSTraversalDirection::Left,
        ZipperDFSTraversalDirection::Up => ZipperDFSTraversalDirection::Right,
      };
      self.iter_direction = ZipperDFSTraversalIterDirection::Backward;
    }

    match self.next_direction {
      ZipperDFSTraversalDirection::Right => match self.zipper.right() {
        ControlFlow::Break(()) => {
          self.next_direction = ZipperDFSTraversalDirection::Up;
          self.next()
        }
        ControlFlow::Continue(()) => ControlFlow::Continue(()),
      },
      ZipperDFSTraversalDirection::Left => match self.zipper.left() {
        ControlFlow::Break(()) => {
          self.next_direction = ZipperDFSTraversalDirection::Up;
          self.next()
        }
        ControlFlow::Continue(()) => {
          self.next_direction = ZipperDFSTraversalDirection::Right;
          ControlFlow::Continue(())
        }
      },
      ZipperDFSTraversalDirection::Up => match self.zipper.up() {
        ControlFlow::Break(()) => {
          self.next_direction = ZipperDFSTraversalDirection::Right;
          ControlFlow::Break(())
        }
        ControlFlow::Continue(direction) => {
          match direction {
            ZipperDirection::Right => {
              self.next_direction = ZipperDFSTraversalDirection::Left;
            }
            ZipperDirection::Left => {
              self.next_direction = ZipperDFSTraversalDirection::Up;
            }
          }
          ControlFlow::Continue(())
        }
      },
    }
  }

  /*
  pub fn take_zipper(&mut self) -> Zipper<T> {}

  pub fn put_zipper(&mut self, &mut zipper: Zipper) {}
  */
}

#[test]
fn test_zipper_dfs_traversal_backward() {
  use crate::trees::boxed::parser::parse_tree;
  use crate::types::LeafValue;

  let tree = parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap();
  let mut zipper = Zipper::new(tree);
  let mut zipper_dfs_traversal = ZipperDFSTraversal::new(zipper);

  let other_tree = parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap();
  let mut other_zipper = Zipper::new(other_tree);

  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.left();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
  zipper_dfs_traversal.next();
  zipper_dfs_traversal.prev();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.left();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
  zipper_dfs_traversal.next();
  zipper_dfs_traversal.prev();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.up();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
  zipper_dfs_traversal.next();
  zipper_dfs_traversal.prev();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.right();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
  zipper_dfs_traversal.next();
  zipper_dfs_traversal.prev();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.up();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
  zipper_dfs_traversal.next();
  zipper_dfs_traversal.prev();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.up();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
  zipper_dfs_traversal.next();
  zipper_dfs_traversal.prev();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.right();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
  zipper_dfs_traversal.next();
  zipper_dfs_traversal.prev();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.left();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
  zipper_dfs_traversal.next();
  zipper_dfs_traversal.prev();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.up();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
  zipper_dfs_traversal.next();
  zipper_dfs_traversal.prev();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.right();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
  zipper_dfs_traversal.next();
  zipper_dfs_traversal.prev();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.up();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
  zipper_dfs_traversal.next();
  zipper_dfs_traversal.prev();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.up();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
  zipper_dfs_traversal.next();
  zipper_dfs_traversal.prev();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);
}

#[test]
fn test_zipper_dfs_traversal() {
  use crate::trees::boxed::parser::parse_tree;
  use crate::types::LeafValue;

  let tree = parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap();
  let mut zipper = Zipper::new(tree);
  let mut zipper_dfs_traversal = ZipperDFSTraversal::new(zipper);

  let other_tree = parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap();
  let mut other_zipper = Zipper::new(other_tree);

  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.left();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.left();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.up();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.right();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.up();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.up();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.right();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.left();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.up();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.right();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.up();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  zipper_dfs_traversal.next();
  other_zipper.up();
  assert_eq!(zipper_dfs_traversal.zipper, other_zipper);

  assert_eq!(zipper_dfs_traversal.next(), ControlFlow::Break(()));
}

#[test]
fn test_tree_zipper_left() {
  use crate::trees::boxed::parser::parse_tree;
  use crate::types::LeafValue;

  let tree = parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap();
  let mut zipper = Zipper::new(tree);

  let result = zipper.left();
  assert_eq!(result, ControlFlow::Continue(()));

  let result = zipper.left();
  assert_eq!(result, ControlFlow::Continue(()));

  let result = zipper.left();
  assert_eq!(result, ControlFlow::Break(()));

  let result = zipper.up();
  assert_eq!(result, ControlFlow::Continue(ZipperDirection::Left));

  let result = zipper.up();
  assert_eq!(result, ControlFlow::Continue(ZipperDirection::Left));

  let result = zipper.up();
  assert_eq!(result, ControlFlow::Break(()));
}

#[test]
fn test_tree_zipper_right() {
  use crate::trees::boxed::parser::parse_tree;
  use crate::types::LeafValue;

  let tree = parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap();
  let mut zipper = Zipper::new(tree);

  let result = zipper.right();
  assert_eq!(result, ControlFlow::Continue(()));

  let result = zipper.right();
  assert_eq!(result, ControlFlow::Continue(()));

  let result = zipper.right();
  assert_eq!(result, ControlFlow::Break(()));
}

#[cfg(test)]
mod test {
  #[test]
  fn test_leaf_not_modified_by_zipper_down() {
    use super::ControlFlow;
    use super::Tree;
    use super::Zipper;
    use crate::trees::boxed::parser::parse_tree;
    use crate::types::LeafValue;

    let tree = parse_tree::<Tree<LeafValue>>("1").unwrap();

    let mut zipper = Zipper::new(tree);
    let expected_zipper = Zipper::Top {
      tree: Some(parse_tree::<Tree<LeafValue>>("1").unwrap()),
    };

    assert_eq!(zipper, expected_zipper);

    let result = zipper.left();
    assert_eq!(result, ControlFlow::Break(()));

    assert_eq!(zipper, expected_zipper);
  }

  #[test]
  fn test_zipper_depth() {
    use super::Tree;
    use super::Zipper;
    use crate::trees::boxed::parser::parse_tree;
    use crate::types::LeafValue;

    let tree = parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap();
    let mut zipper = Zipper::new(tree);

    assert_eq!(zipper.get_depth(), 0);

    zipper.left();
    assert_eq!(zipper.get_depth(), 1);

    zipper.left();
    assert_eq!(zipper.get_depth(), 2);

    zipper.left();
    assert_eq!(zipper.get_depth(), 2);

    zipper.up();
    assert_eq!(zipper.get_depth(), 1);

    zipper.up();
    assert_eq!(zipper.get_depth(), 0);

    zipper.up();
    assert_eq!(zipper.get_depth(), 0);
  }

  #[test]
  fn test_zipper_attach() {
    use super::Tree;
    use super::Zipper;
    use crate::trees::boxed::parser::parse_tree;
    use crate::types::LeafValue;

    let tree = parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap();
    let mut zipper = Zipper::new(tree);

    zipper.left();
    zipper.left();

    zipper.attach(parse_tree::<Tree<LeafValue>>("[4,7]").unwrap());

    zipper.up();
    zipper.up();

    zipper.right();
    zipper.right();

    zipper.attach(parse_tree::<Tree<LeafValue>>("[1,2]").unwrap());

    let modified_tree = zipper.to_tree();
    assert_eq!(zipper, Zipper::Emptied);
    assert_eq!(
      modified_tree,
      parse_tree::<Tree<LeafValue>>("[[[4,7],9],[8,[1,2]]]").unwrap()
    );
  }
}
