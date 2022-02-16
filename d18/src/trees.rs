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
  fn with_depth_iter(&self) -> WithDepthIter<'_, T> {
    WithDepthIter {
      curr_depth: 0,
      next_visit: vec![self],
      parent: None,
    }
  }
}

// Modify to return depth, then use a while let loop to iterate over this
// iterator:
// https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html.
// Then store left leaf's parent and right leaf's parent in a local variable. If
// the condition is met, then all of the required data is available.
// Modify to be a mutable iterator.
struct WithDepthIter<'a, T> {
  curr_depth: usize,
  next_visit: Vec<&'a Tree<T>>,
  parent: Option<Box<WithDepthIter<'a, T>>>,
}

impl<T> WithDepthIter<'_, T> {
  fn get_curr_depth(&self) -> usize {
    self.curr_depth
  }
}

impl<T> Default for WithDepthIter<'_, T> {
  fn default() -> Self {
    WithDepthIter {
      curr_depth: 0,
      next_visit: vec![],
      parent: None,
    }
  }
}

impl<'a, T> Iterator for WithDepthIter<'a, T> {
  type Item = (usize, &'a Tree<T>);

  fn next(&mut self) -> Option<Self::Item> {
    if self.next_visit.len() > 2 {
      panic!("Logic error")
    }

    match self.next_visit.pop() {
      // subtree found, visit subtree
      Some(subtree) => {
        // advance iterator
        let curr_depth = self.curr_depth;
        match subtree {
          Tree::Leaf(_) => (), // do nothing, nothing to recurse into
          Tree::NonLeaf {
            left: left_subtree,
            right: right_subtree,
          } => {
            *self = WithDepthIter {
              curr_depth: self.curr_depth + 1,
              next_visit: vec![right_subtree, left_subtree],
              parent: Some(Box::new(mem::take(self))),
            }
          }
        }
        Some((curr_depth, subtree))
      }

      // all subtrees visited, pop back up
      None => match self.parent.take() {
        Some(parent) => {
          // get parent and go onto next visit
          *self = *parent;
          self.next()
        }
        // back at the root, so finish iterating
        None => None,
      },
    }
  }
}

#[test]
fn test_with_depth_iterator() {
  use crate::parser::parse_tree;
  use crate::types::SnailfishNumber;

  let mut expected_subtrees = vec!["[[1,9],[8,5]]", "[1,9]", "1", "9", "[8,5]", "8", "5"];
  expected_subtrees.reverse();
  let mut expected_depths = vec![0, 1, 2, 2, 1, 2, 2];
  expected_depths.reverse();

  let tree: SnailfishNumber = parse_tree("[[1,9],[8,5]]").unwrap();
  for (depth, subtree) in tree.with_depth_iter() {
    let expected_subtree: SnailfishNumber = parse_tree(expected_subtrees.pop().unwrap()).unwrap();
    let expected_depth = expected_depths.pop().unwrap();

    assert_eq!(expected_depth, depth);
    assert_eq!(&expected_subtree, subtree);
    println!("{:?} -> {:?}", depth, subtree);
  }
}

// trait WindowedIterator
