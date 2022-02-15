use crate::trees::Tree;
use crate::types::*;

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
    (remainder, SnailfishNumber::Leaf(number))
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
      Tree::NonLeaf {
        left: Box::new(left_subtree),
        right: Box::new(right_subtree),
      },
    )
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
    ("1", Tree::Leaf(1)),
    (
      "[1,2]",
      Tree::NonLeaf {
        left: Box::new(Tree::Leaf(1)),
        right: Box::new(Tree::Leaf(2)),
      },
    ),
    (
      "[[1,2],3]",
      Tree::NonLeaf {
        left: Box::new(Tree::NonLeaf {
          left: Box::new(Tree::Leaf(1)),
          right: Box::new(Tree::Leaf(2)),
        }),
        right: Box::new(Tree::Leaf(3)),
      },
    ),
    (
      "[9,[8,7]]",
      Tree::NonLeaf {
        left: Box::new(Tree::Leaf(9)),
        right: Box::new(Tree::NonLeaf {
          left: Box::new(Tree::Leaf(8)),
          right: Box::new(Tree::Leaf(7)),
        }),
      },
    ),
    (
      "[[1,9],[8,5]]",
      Tree::NonLeaf {
        left: Box::new(Tree::NonLeaf {
          left: Box::new(Tree::Leaf(1)),
          right: Box::new(Tree::Leaf(9)),
        }),
        right: Box::new(Tree::NonLeaf {
          left: Box::new(Tree::Leaf(8)),
          right: Box::new(Tree::Leaf(5)),
        }),
      },
    ),
  ];
  for testcase in testcases {
    let input = testcase.0;
    let expected = testcase.1;
    match parse_tree(input) {
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

pub fn parse_input(input: &str) -> Result<Vec<SnailfishNumber>, &'static str> {
  input.lines().map(|line| parse_tree(line)).collect()
}
