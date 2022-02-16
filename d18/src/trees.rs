// this one has an example of a mutable iterator:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=985c685d5809121fc93c3bdeb64fa755

use std::mem;

// How to handle leaf nodes?
// Where to store value?
#[derive(Debug)]
pub enum Tree<T> {
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

impl<T> Tree<T> {
  fn iter(&self) -> TreeIter<'_, T> {
    TreeIter {
      curr_depth: 0,
      next_depth: 0,
      next_visit: vec![self],
      parent: None,
    }
  }
}

// Modify to return depth, change to regular iterator, then use a while let loop
// to iterate over this iterator:
// https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html.
// Then store left leaf's parent and right leaf's parent in a local variable. If
// the condition is met, then all of the required data is available.
// Modify to be a mutable iterator.
struct TreeIter<'a, T> {
  curr_depth: usize,
  next_depth: usize,
  next_visit: Vec<&'a Tree<T>>,
  parent: Option<Box<TreeIter<'a, T>>>,
}

impl<T> TreeIter<'_, T> {
  fn get_curr_depth(&self) -> usize {
    self.curr_depth
  }
}

impl<T> Default for TreeIter<'_, T> {
  fn default() -> Self {
    TreeIter {
      curr_depth: 0,
      next_depth: 0,
      next_visit: vec![],
      parent: None,
    }
  }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
  type Item = &'a Tree<T>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.next_visit.len() > 2 {
      panic!("Logic error")
    }

    match self.next_visit.pop() {
      // subtree found, visit subtree
      Some(subtree) => {
        self.curr_depth = self.next_depth;

        // advance iterator
        // TODO: This is wrong.. because it is not used. It means that curr_depth is not correct.
        // let curr_depth = self.curr_depth;
        match subtree {
          Tree::Leaf(_) => (), // do nothing, nothing to recurse into
          Tree::NonLeaf {
            left: left_subtree,
            right: right_subtree,
          } => {
            *self = TreeIter {
              curr_depth: self.curr_depth,
              next_depth: self.curr_depth + 1,
              next_visit: vec![right_subtree, left_subtree],
              parent: Some(Box::new(mem::take(self))),
            }
          }
        }
        Some(subtree)
      }

      // all subtrees visited, pop back up
      None => match self.parent.take() {
        Some(parent) => {
          // get parent and go onto next visit
          *self = *parent;
          self.next()
        }
        // back at the root, so finish iterating
        None => {
          self.curr_depth = 0;
          None
        }
      },
    }
  }
}

#[test]
fn test_tree_iter() {
  use crate::parser::parse_tree;
  use crate::types::SnailfishNumber;

  let mut expected_subtrees = vec!["[[1,9],[8,5]]", "[1,9]", "1", "9", "[8,5]", "8", "5"];
  expected_subtrees.reverse();
  let mut expected_depths = vec![0, 1, 2, 2, 1, 2, 2];
  expected_depths.reverse();

  let tree: SnailfishNumber = parse_tree("[[1,9],[8,5]]").unwrap();
  let mut tree_iter = tree.iter();
  while let Some(subtree) = tree_iter.next() {
    let expected_subtree: SnailfishNumber = parse_tree(expected_subtrees.pop().unwrap()).unwrap();
    let expected_depth = expected_depths.pop().unwrap();
    let depth = tree_iter.get_curr_depth();
    assert_eq!(expected_depth, depth);
    assert_eq!(&expected_subtree, subtree);
    println!("{:?} -> {:?}", depth, subtree);
  }
  assert_eq!(tree_iter.get_curr_depth(), 0);
}

#[test]
fn test_mut_iter() {
  unimplemented!();
}
