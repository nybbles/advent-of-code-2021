use crate::types::*;

use trees::tr;
use trees::Node;

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
    (remainder, tr(Some(number)))
  })
}

fn subtree(input: &str) -> IResult<&str, SnailfishNumber> {
  delimited(
    opening_delimiter,
    separated_pair(tree, subtree_separator, tree),
    closing_delimiter,
  )(input)
  .map(|(remainder, (left_subtree, right_subtree))| {
    (remainder, tr(None) / left_subtree / right_subtree)
  })
}

fn tree(input: &str) -> IResult<&str, SnailfishNumber> {
  alt((leaf, subtree))(input)
}

pub fn parse_tree(input: &str) -> Result<SnailfishNumber, &'static str> {
  let (remainder, parsed) = tree(input).unwrap();
  if remainder.is_empty() {
    Ok(parsed)
  } else {
    Err("Parse error: did not consume whole input")
  }
}

#[test]
fn test_parse_simple_input() {
  let testcases: Vec<(&str, SnailfishNumber)> = vec![
    ("1", tr(Some(1))),
    ("[1,2]", tr(None) / tr(Some(1)) / tr(Some(2))),
    (
      "[[1,2],3]",
      tr(None) / (tr(None) / tr(Some(1)) / tr(Some(2))) / tr(Some(3)),
    ),
    (
      "[9,[8,7]]",
      tr(None) / tr(Some(9)) / (tr(None) / tr(Some(8)) / tr(Some(7))),
    ),
    (
      "[[1,9],[8,5]]",
      tr(None) / (tr(None) / tr(Some(1)) / tr(Some(9))) / (tr(None) / tr(Some(8)) / tr(Some(5))),
    ),
  ];
  for testcase in testcases {
    let input = testcase.0;
    let expected = testcase.1;
    match parse_tree(input) {
      Ok(tree) => {
        let result = trees_eq(tree.root(), expected.root());
        if !result {
          println!("{}, {:?} <--> {:?}", input, tree, expected);
        }
        assert!(result)
      }
      Err(_msg) => assert!(false),
    }
  }
}

pub fn parse_input(input: &str) -> Result<Vec<SnailfishNumber>, &'static str> {
  input.lines().map(|line| parse_tree(line)).collect()
}
