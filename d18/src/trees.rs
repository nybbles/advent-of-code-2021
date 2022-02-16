pub mod boxed;
pub mod parsing_utils;
pub mod refcell;

pub trait TreeBuilder<U> {
  type Tree;

  fn leaf(value: U) -> Self;
  fn non_leaf(left: Self, right: Self) -> Self;
  fn get_tree(&mut self) -> Self::Tree;
}
