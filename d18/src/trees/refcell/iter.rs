use crate::trees::refcell::{SubtreeRef, Tree};

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub struct TreeIter<T> {
  pub curr_depth: usize,
  pub next_depth: usize,
  pub next_visit: Vec<SubtreeRef<T>>,
  pub parent: Option<Box<TreeIter<T>>>,
}

impl<T> TreeIter<T> {
  fn get_curr_depth(&self) -> usize {
    self.curr_depth
  }
}

impl<T> Default for TreeIter<T> {
  fn default() -> Self {
    TreeIter {
      curr_depth: 0,
      next_depth: 0,
      next_visit: vec![],
      parent: None,
    }
  }
}

impl<'a, T> Iterator for TreeIter<T> {
  type Item = SubtreeRef<T>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.next_visit.len() > 2 {
      panic!("Logic error")
    }

    match self.next_visit.pop() {
      // subtree found, visit subtree
      Some(subtree_ref) => {
        self.curr_depth = self.next_depth;

        // advance iterator
        let subtree = &*subtree_ref.borrow();
        match subtree {
          Tree::Leaf(_) => (), // do nothing, nothing to recurse into
          Tree::NonLeaf {
            left: left_subtree_ref,
            right: right_subtree_ref,
          } => {
            *self = TreeIter {
              curr_depth: self.curr_depth,
              next_depth: self.curr_depth + 1,
              next_visit: vec![Rc::clone(right_subtree_ref), Rc::clone(left_subtree_ref)],
              parent: Some(Box::new(mem::take(self))),
            }
          }
        }
        Some(Rc::clone(&subtree_ref))
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
  use crate::trees::refcell::parser::parse_tree;
  use crate::types::LeafValue;

  let mut expected_subtrees = vec!["[[1,9],[8,5]]", "[1,9]", "1", "9", "[8,5]", "8", "5"];
  expected_subtrees.reverse();
  let mut expected_depths = vec![0, 1, 2, 2, 1, 2, 2];
  expected_depths.reverse();

  let tree = Rc::new(RefCell::new(
    parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap(),
  ));
  let mut tree_iter = Tree::iter(tree);

  while let Some(subtree) = tree_iter.next() {
    let expected_subtree = parse_tree::<Tree<LeafValue>>(expected_subtrees.pop().unwrap()).unwrap();
    let expected_depth = expected_depths.pop().unwrap();
    let depth = tree_iter.get_curr_depth();
    assert_eq!(expected_depth, depth);
    assert_eq!(expected_subtree, *subtree.borrow());
  }
  assert_eq!(tree_iter.get_curr_depth(), 0);
}

#[test]
fn test_tree_iter_with_mutable_borrows() {
  use crate::trees::refcell::parser::parse_tree;
  use crate::types::LeafValue;

  let tree = Rc::new(RefCell::new(
    parse_tree::<Tree<LeafValue>>("[[1,9],[8,5]]").unwrap(),
  ));
  let mut tree_iter = Tree::iter(tree);

  let root = tree_iter.next().unwrap();
  let subtree = tree_iter.next().unwrap();

  {
    let borrowed_root = root.borrow();
    let mut borrowed_subtree = subtree.borrow_mut();

    if let Tree::NonLeaf { ref mut left, .. } = &mut *borrowed_subtree {
      *left = SubtreeRef::new(RefCell::new(Tree::Leaf(30)));
    } else {
      assert!(false);
    }
  }
}
