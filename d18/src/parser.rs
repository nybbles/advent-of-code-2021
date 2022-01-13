use crate::types::*;

use trees::tr;
use trees::Tree;

use nom::{
  character::complete::{char, digit1, one_of},
  combinator::recognize,
  multi::{many0, many1},
  sequence::terminated,
  IResult,
};

// fn delimited_expr(input: &str) -> IResult<&str, &str>

fn parse_tree(input: &str) -> Tree<NodeValue> {
  Tree::new(NodeValue(None))
}

#[test]
fn test_parse_simple_input() {
  let testcases: Vec<(&str, Tree<NodeValue>)> = vec![(
    "[1,2]",
    tr(NodeValue(None)) / tr(NodeValue(Some(1))) / tr(NodeValue(Some(2))),
  )];
  for testcase in testcases {
    let input = testcase.0;
    let expected: Tree<NodeValue> = testcase.1;
    let tree = parse_tree(input);
    assert_eq!(tree.to_string(), expected.to_string())
  }
}
