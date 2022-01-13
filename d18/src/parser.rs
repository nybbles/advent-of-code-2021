use crate::types::*;

use trees::tr;
use trees::Tree;

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

fn leaf(input: &str) -> IResult<&str, SnailfishNumber> {
  one_of("0123456789")(input).map(|(remainder, matched)| {
    let number = matched.to_digit(10).unwrap();
    (remainder, tr(NodeValue(Some(number))))
  })
}

fn subtree(input: &str) -> IResult<&str, SnailfishNumber> {
  delimited(
    opening_delimiter,
    separated_pair(tree, subtree_separator, tree),
    closing_delimiter,
  )(input)
  .map(|(remainder, (left_subtree, right_subtree))| {
    (
      remainder,
      tr(NodeValue(None)) / left_subtree / right_subtree,
    )
  })
}

fn tree(input: &str) -> IResult<&str, SnailfishNumber> {
  alt((leaf, subtree))(input)
}

fn parse_tree(input: &str) -> Result<Tree<NodeValue>, &'static str> {
  let (remainder, parsed) = tree(input).unwrap();
  if remainder.is_empty() {
    Ok(parsed)
  } else {
    Err("Parse error: did not consume whole input")
  }
}

#[test]
fn test_parse_simple_input() {
  let testcases: Vec<(&str, Tree<NodeValue>)> = vec![
    ("1", tr(NodeValue(Some(1)))),
    (
      "[1,2]",
      tr(NodeValue(None)) / tr(NodeValue(Some(1))) / tr(NodeValue(Some(2))),
    ),
    (
      "[[1,2],3]",
      tr(NodeValue(None))
        / (tr(NodeValue(None)) / tr(NodeValue(Some(1))) / tr(NodeValue(Some(2))))
        / tr(NodeValue(Some(3))),
    ),
    (
      "[9,[8,7]]",
      tr(NodeValue(None))
        / tr(NodeValue(Some(9)))
        / (tr(NodeValue(None)) / tr(NodeValue(Some(8))) / tr(NodeValue(Some(7)))),
    ),
    (
      "[[1,9],[8,5]]",
      tr(NodeValue(None))
        / (tr(NodeValue(None)) / tr(NodeValue(Some(1))) / tr(NodeValue(Some(9))))
        / (tr(NodeValue(None)) / tr(NodeValue(Some(8))) / tr(NodeValue(Some(5)))),
    ),
  ];
  for testcase in testcases {
    let input = testcase.0;
    let expected: Tree<NodeValue> = testcase.1;
    match parse_tree(input) {
      Ok(tree) => assert_eq!(tree.to_string(), expected.to_string()),
      Err(_msg) => assert!(false),
    }
  }
}
