// this one has an example of a mutable iterator:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=985c685d5809121fc93c3bdeb64fa755

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

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

// Then store left leaf's parent and right leaf's parent in a local variable. If
// the condition is met, then all of the required data is available.
// Modify to be a mutable iterator.
struct TreeIter<T> {
  curr_depth: usize,
  next_depth: usize,
  next_visit: Vec<SubtreeRef<T>>,
  parent: Option<Box<TreeIter<T>>>,
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
  use crate::parser::parse_tree;
  use crate::types::SnailfishNumber;

  let mut expected_subtrees = vec!["[[1,9],[8,5]]", "[1,9]", "1", "9", "[8,5]", "8", "5"];
  expected_subtrees.reverse();
  let mut expected_depths = vec![0, 1, 2, 2, 1, 2, 2];
  expected_depths.reverse();

  let tree = Rc::new(RefCell::new(parse_tree("[[1,9],[8,5]]").unwrap()));
  let mut tree_iter = Tree::iter(tree);

  while let Some(subtree) = tree_iter.next() {
    let expected_subtree = parse_tree(expected_subtrees.pop().unwrap()).unwrap();
    let expected_depth = expected_depths.pop().unwrap();
    let depth = tree_iter.get_curr_depth();
    assert_eq!(expected_depth, depth);
    assert_eq!(expected_subtree, *subtree.borrow());
  }
  assert_eq!(tree_iter.get_curr_depth(), 0);
}

#[test]
fn test_tree_iter_with_mutable_borrows() {
  use crate::parser::parse_tree;
  use crate::types::SnailfishNumber;

  let tree = Rc::new(RefCell::new(parse_tree("[[1,9],[8,5]]").unwrap()));
  let mut tree_iter = Tree::iter(tree);

  let root = tree_iter.next().unwrap();
  let subtree = tree_iter.next().unwrap();

  println!("Step 0");
  println!("{:?}", root.borrow());
  println!("{:?}", subtree.borrow());

  {
    let mut borrowed_root = root.borrow_mut();
    let mut borrowed_subtree = subtree.borrow_mut();

    println!("Step 1");
    println!("{:?}", borrowed_root);
    println!("{:?}", borrowed_subtree);

    if let Tree::NonLeaf {
      ref left,
      ref right,
    } = &*borrowed_root
    {
      println!("WTF");
      println!("{:?}", right.borrow());
      println!("FTW");
    } else {
      assert!(false);
    }

    if let Tree::NonLeaf { ref mut left, .. } = &mut *borrowed_subtree {
      *left = SubtreeRef::new(RefCell::new(Tree::Leaf(30)));
    } else {
      assert!(false);
    }

    println!("Step 2");
    println!("{:?}", borrowed_root);
    println!("{:?}", borrowed_subtree);
  }

  println!("Step 3");
  println!("{:?}", root.borrow());
  println!("{:?}", subtree.borrow());
}
