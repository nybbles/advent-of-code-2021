use crate::trees::refcell::Tree;
use crate::trees::TreeBuilder;
use crate::types::LeafValue;

use nom::{
  branch::alt,
  character::complete::{char, one_of},
  sequence::{delimited, separated_pair},
  IResult,
};

fn opening_delimiter(input: &str) -> IResult<&str, char> {
  char('[')(input)
}

fn closing_delimiter(input: &str) -> IResult<&str, char> {
  char(']')(input)
}

fn subtree_separator(input: &str) -> IResult<&str, char> {
  char(',')(input)
}

fn leaf<TB: TreeBuilder<LeafValue>>(input: &str) -> IResult<&str, TB> {
  one_of("0123456789")(input).map(|(remainder, matched)| {
    let number = matched.to_digit(10).unwrap();
    (remainder, TB::leaf(number))
  })
}

fn subtree<TB: TreeBuilder<LeafValue>>(input: &str) -> IResult<&str, TB> {
  delimited(
    opening_delimiter,
    separated_pair(tree, subtree_separator, tree),
    closing_delimiter,
  )(input)
  .map(|(remainder, (left_subtree, right_subtree))| {
    (remainder, TB::non_leaf(left_subtree, right_subtree))
  })
}

fn tree<TB: TreeBuilder<LeafValue>>(input: &str) -> IResult<&str, TB> {
  alt((leaf, subtree))(input)
}

pub fn parse_tree<TB: TreeBuilder<LeafValue>>(input: &str) -> Result<TB::Tree, &'static str> {
  let (remainder, mut parsed): (&str, TB) = tree(input).unwrap();
  if remainder.is_empty() {
    Ok(parsed.get_tree())
  } else {
    Err("Parse error: did not consume whole input")
  }
}

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
