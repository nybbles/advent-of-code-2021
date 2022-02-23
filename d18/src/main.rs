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

fn snailfish_reduce(number: &mut SnailfishNumber) {
    loop {
        if !snailfish_find_and_explode(number) && !snailfish_find_and_split(number) {
            break;
        }
    }
}

fn snailfish_add_and_reduce(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
    let mut reduce = snailfish_add(left, right);
    snailfish_reduce(&mut reduce);
    reduce
}

#[test]
fn test_snailfish_add_and_reduce() {
    let testcases = vec![(
        ("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"),
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
    )];

    for ((left_input_str, right_input_str), expected_output_str) in testcases.iter() {
        let left = parse_tree::<SnailfishNumber>(left_input_str).unwrap();
        let right = parse_tree::<SnailfishNumber>(right_input_str).unwrap();
        let expected_output = parse_tree::<SnailfishNumber>(expected_output_str).unwrap();

        let result = snailfish_add_and_reduce(left, right);
        assert_eq!(result, expected_output);
    }
}

fn snailfish_add_and_reduce_all(mut numbers: Vec<SnailfishNumber>) -> SnailfishNumber {
    assert!(!numbers.is_empty());
    numbers.reverse();

    let mut result = numbers.pop().unwrap();

    while let Some(right) = numbers.pop() {
        result = snailfish_add_and_reduce(result, right);
    }

    result
}

#[test]
fn test_snailfish_add_and_reduce_all() {
    let testcases = vec![
        (
            vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]"],
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        ),
        (
            vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"],
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        ),
        (
            vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"],
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        ),
        (
            vec![
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[2,9]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[4,2],2],6],[8,7]]",
            ],
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ),
    ];

    for (numbers_str, expected_output_str) in testcases.iter() {
        let numbers = numbers_str
            .iter()
            .map(|number_str| parse_tree::<SnailfishNumber>(number_str).unwrap())
            .collect();
        let expected_output = parse_tree::<SnailfishNumber>(expected_output_str).unwrap();

        let result = snailfish_add_and_reduce_all(numbers);
        assert_eq!(result, expected_output);
    }
}

fn snailfish_magnitude(input: &SnailfishNumber) -> LeafValue {
    match input {
        Tree::Leaf(value) => *value,
        Tree::NonLeaf { left, right } => {
            let left_result = snailfish_magnitude(left);
            let right_result = snailfish_magnitude(right);
            3 * left_result + 2 * right_result
        }
    }
}

#[test]
fn test_snailfish_magnitude() {
    let testcases = vec![
        ("[[1,2],[[3,4],5]]", 143),
        ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
        ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
        ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
        ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
        (
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        ),
    ];

    for (input_str, expected) in testcases.iter() {
        let input = parse_tree::<SnailfishNumber>(input_str).unwrap();
        let output = snailfish_magnitude(&input);
        assert_eq!(output, *expected);
    }
}

#[test]
fn test_final_testcase() {
    let numbers = vec![
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
        "[[[5,[2,8]],4],[5,[[9,9],0]]]",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
        "[[[[5,4],[7,7]],8],[[8,3],8]]",
        "[[9,3],[[9,9],[6,[4,9]]]]",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    ]
    .iter()
    .map(|number_str| parse_tree::<SnailfishNumber>(number_str).unwrap())
    .collect();

    let expected_final_sum = parse_tree::<SnailfishNumber>(
        "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
    )
    .unwrap();

    let result = snailfish_add_and_reduce_all(numbers);
    assert_eq!(result, expected_final_sum);

    let expected_magnitude = 4140;
    assert_eq!(snailfish_magnitude(&result), expected_magnitude);
}

fn main() {
    println!("Hello, world!");
}
