// Adding -> requires constructing a new tree out of two subtrees.
// Exploding -> requires finding the leftmost node that is at depth at least 4,
// and then finding the leaves immediately on the left and the right. These
// leaves might be in an entirely different branch of the tree.
// Splitting -> requires replacing a leaf node with a subtree with just two
// Finding nodes to be split or exploded requires traversing the tree
// depth-first starting with the left subtree first, until an actionable node is
// found.

// Need to parse string into a tree

mod trees;
mod types;

use crate::trees::boxed::parser::parse_tree;
use crate::trees::boxed::zipper::{Zipper, ZipperDFSTraversal};
use crate::trees::boxed::Tree;
use crate::types::*;
use std::mem;

use std::ops::ControlFlow;

fn snailfish_add(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
    SnailfishNumber::NonLeaf {
        left: Box::new(left),
        right: Box::new(right),
    }
}

#[test]
fn test_snailfish_add() {
    let tree0 = parse_tree::<SnailfishNumber>("1").unwrap();
    let tree1 = parse_tree::<SnailfishNumber>("[2,3]").unwrap();
    let result = snailfish_add(tree0, tree1);
    let expected = parse_tree::<SnailfishNumber>("[1,[2,3]]").unwrap();
    assert!(result == expected)
}

enum ReduceAction {
    Explode,
    Split,
}

/*
fn snailfish_find_next_reduce_action(input: SnailfishNumber) -> Option<ReduceAction> {
    if input.has_no_child() {
        return None;
    }
    Some(ReduceAction::Explode)
}
*/

const EXPLODE_DEPTH: usize = 4;

fn snailfish_find_and_explode(input: &mut SnailfishNumber) -> bool {
    let mut zipper_dfs_traversal = ZipperDFSTraversal::new(Zipper::new(mem::take(input)));

    while zipper_dfs_traversal.zipper.get_depth() < EXPLODE_DEPTH
        || zipper_dfs_traversal.zipper.focused_subtree().is_leaf()
    {
        match zipper_dfs_traversal.next() {
            ControlFlow::Break(()) => {
                *input = zipper_dfs_traversal.zipper.to_tree();
                return false;
            }
            ControlFlow::Continue(()) => (),
        }
    }

    assert!(zipper_dfs_traversal.zipper.get_depth() >= EXPLODE_DEPTH);
    snailfish_explode(&mut zipper_dfs_traversal);

    *input = zipper_dfs_traversal.zipper.to_tree();
    true
}

fn snailfish_explode(input: &mut ZipperDFSTraversal<LeafValue>) {
    // extract exploding pair values
    let (left, right): (LeafValue, LeafValue) = match input.zipper.focused_subtree() {
        Tree::NonLeaf { left, right } => {
            match (&*left as &SnailfishNumber, &*right as &SnailfishNumber) {
                (Tree::Leaf(left), Tree::Leaf(right)) => (*left, *right),
                _ => panic!("Logic error"),
            }
        }
        _ => {
            println!("{:?}", input.zipper.focused_subtree());
            panic!("Logic error")
        }
    };

    // replace exploding pair with 0
    input.zipper.attach(Tree::Leaf(0));

    // go to regular number on left and update number
    while input.prev() != ControlFlow::Break(()) {
        let left_regular_number = match input.zipper.focused_subtree() {
            Tree::Leaf(value) => *value,
            _ => continue,
        };
        input.zipper.attach(Tree::Leaf(left_regular_number + left));
        break;
    }

    // go back to exploded pair
    loop {
        let result = input.next();
        assert_ne!(result, ControlFlow::Break(()));

        match input.zipper.focused_subtree() {
            Tree::Leaf(value) => {
                assert_eq!(*value, 0);
                break;
            }
            _ => continue,
        };
    }

    // go to regular number on right and update number
    while input.next() != ControlFlow::Break(()) {
        let right_regular_number = match input.zipper.focused_subtree() {
            Tree::Leaf(value) => *value,
            _ => continue,
        };
        input
            .zipper
            .attach(Tree::Leaf(right_regular_number + right));

        break;
    }
}

#[test]
fn test_snailfish_find_and_explode() {
    let testcases = vec![
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        (
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ),
        (
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ),
    ];

    for (input_str, expected_output_str) in testcases.iter() {
        let mut input = parse_tree::<SnailfishNumber>(input_str).unwrap();
        let expected_output = parse_tree::<SnailfishNumber>(expected_output_str).unwrap();

        assert!(snailfish_find_and_explode(&mut input));
        assert_eq!(input, expected_output);
    }
}

const SPLIT_THRESHOLD: LeafValue = 10;

fn snailfish_find_and_split(input: &mut SnailfishNumber) -> bool {
    let mut zipper_dfs_traversal = ZipperDFSTraversal::new(Zipper::new(mem::take(input)));

    loop {
        match zipper_dfs_traversal.next() {
            ControlFlow::Break(()) => {
                *input = zipper_dfs_traversal.zipper.to_tree();
                return false;
            }
            ControlFlow::Continue(()) => (),
        }

        if !zipper_dfs_traversal.zipper.focused_subtree().is_leaf() {
            continue;
        }

        let leaf_value = match zipper_dfs_traversal.zipper.focused_subtree() {
            Tree::Leaf(value) => value,
            _ => panic!("Logic error"),
        };

        if *leaf_value >= SPLIT_THRESHOLD {
            break;
        } else {
            continue;
        }
    }

    snailfish_split(&mut zipper_dfs_traversal.zipper);

    *input = zipper_dfs_traversal.zipper.to_tree();
    true
}

fn snailfish_split(input: &mut Zipper<LeafValue>) {
    let leaf_value = match input.focused_subtree() {
        Tree::Leaf(value) => *value,
        _ => panic!("Logic error"),
    };

    input.attach(Tree::NonLeaf {
        left: Box::new(Tree::Leaf(
            (leaf_value as f32 / 2 as f32).floor() as LeafValue
        )),
        right: Box::new(Tree::Leaf(
            (leaf_value as f32 / 2 as f32).ceil() as LeafValue
        )),
    });
}

#[test]
fn test_snailfish_find_and_split() {
    let testcases = vec![("[10,1]", "[[5,5],1]"), ("[5,[11,2]]", "[5,[[5,6],2]]")];

    for (input_str, expected_output_str) in testcases.iter() {
        let mut input = parse_tree::<SnailfishNumber>(input_str).unwrap();
        let expected_output = parse_tree::<SnailfishNumber>(expected_output_str).unwrap();

        assert!(snailfish_find_and_split(&mut input));
        assert_eq!(input, expected_output);
    }
}

// Need to modify the nested pair
// Need to modify first regular number on left and on right of the nested pair

// fn snailfish_split()
// Need to modify the regular number and replace with a pair

// fn snailfish_reduce(number: SnailfishNumber) -> SnailfishNumber {}

/*
#[test]
fn test_snailfish_add_and_reduce_tc01() {
    let input = "[1,1]
[2,2]
[3,3]
[4,4]";
    match parse_input(input) {
        Ok(_numbers) => assert!(true),
        Err(_msg) => assert!(false),
    }
}
*/

fn main() {
    println!("Hello, world!");
}
