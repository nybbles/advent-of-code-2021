pub use crate::trees::parsing_utils::parse_tree;
use crate::trees::refcell::Tree;
use crate::trees::TreeBuilder;
use crate::types::LeafValue;

#[test]
fn test_parse_simple_input() {
  let testcases: Vec<(&str, Tree<LeafValue>)> = vec![
    ("1", Tree::Leaf(1)),
    ("[1,2]", Tree::non_leaf(Tree::Leaf(1), Tree::Leaf(2))),
    (
      "[[1,2],3]",
      Tree::non_leaf(Tree::non_leaf(Tree::Leaf(1), Tree::Leaf(2)), Tree::Leaf(3)),
    ),
    (
      "[9,[8,7]]",
      Tree::non_leaf(Tree::Leaf(9), Tree::non_leaf(Tree::Leaf(8), Tree::Leaf(7))),
    ),
    (
      "[[1,9],[8,5]]",
      Tree::non_leaf(
        Tree::non_leaf(Tree::Leaf(1), Tree::Leaf(9)),
        Tree::non_leaf(Tree::Leaf(8), Tree::Leaf(5)),
      ),
    ),
  ];
  for testcase in testcases {
    let input = testcase.0;
    let expected = testcase.1;
    match parse_tree::<Tree<LeafValue>>(input) {
      Ok(tree) => {
        let result = tree == expected;
        if !result {
          println!("{}, {:?} <--> {:?}", input, tree, expected);
        }
        assert!(result)
      }
      Err(_msg) => assert!(false),
    }
  }
}

pub fn parse_input(input: &str) -> Result<Vec<Tree<LeafValue>>, &'static str> {
  input
    .lines()
    .map(|line| parse_tree::<Tree<LeafValue>>(line))
    .collect()
}
