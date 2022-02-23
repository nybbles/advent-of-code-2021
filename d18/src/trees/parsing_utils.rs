use crate::trees::TreeBuilder;
use crate::types::LeafValue;

use nom::{
  branch::alt,
  character::complete::{char, digit1},
  sequence::{delimited, separated_pair},
  IResult,
};

pub fn opening_delimiter(input: &str) -> IResult<&str, char> {
  char('[')(input)
}

pub fn closing_delimiter(input: &str) -> IResult<&str, char> {
  char(']')(input)
}

pub fn subtree_separator(input: &str) -> IResult<&str, char> {
  char(',')(input)
}

pub fn leaf<TB: TreeBuilder<LeafValue>>(input: &str) -> IResult<&str, TB> {
  digit1(input).map(|(remainder, matched)| {
    let number = matched.parse().unwrap();
    (remainder, TB::leaf(number))
  })
}

pub fn subtree<TB: TreeBuilder<LeafValue>>(input: &str) -> IResult<&str, TB> {
  delimited(
    opening_delimiter,
    separated_pair(tree, subtree_separator, tree),
    closing_delimiter,
  )(input)
  .map(|(remainder, (left_subtree, right_subtree))| {
    (remainder, TB::non_leaf(left_subtree, right_subtree))
  })
}

pub fn tree<TB: TreeBuilder<LeafValue>>(input: &str) -> IResult<&str, TB> {
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
