use trees::tr;
use trees::{Node, Tree};

// Adding -> requires constructing a new tree out of two subtrees.
// Exploding -> requires finding the leftmost node that is at depth at least 4,
// and then finding the leaves immediately on the left and the right. These
// leaves might be in an entirely different branch of the tree.
// Splitting -> requires replacing a leaf node with a subtree with just two
// Finding nodes to be split or exploded requires traversing the tree
// depth-first starting with the left subtree first, until an actionable node is
// found.

// Need to parse string into a tree

mod parser;
mod sliding_window;
mod types;

use crate::parser::*;
use crate::sliding_window::*;
use crate::types::*;

fn snailfish_add(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
    tr(None) / left / right
}

#[test]
fn test_snailfish_add() {
    let tree0 = parse_tree("1").unwrap();
    let tree1 = parse_tree("[2,3]").unwrap();
    let result = snailfish_add(tree0, tree1);
    let expected = tr(None) / tr(Some(1)) / (tr(None) / tr(Some(2)) / tr(Some(3)));
    assert!(trees_eq(result.root(), expected.root()))
}

// fn snailfish_reduce(number: SnailfishNumber) -> SnailfishNumber {}

#[test]
fn test_snailfish_add_and_reduce_tc01() {
    let input = "[1,1]
[2,2]
[3,3]
[4,4]";
    match parse_input(input) {
        Ok(numbers) => assert!(true),
        Err(msg) => assert!(false),
    }
}

fn main() {
    println!("Hello, world!");
}
