#! python

# assumption: the folds are always through the middle of the paper
# given the fold, need to take the moving half and compute the new layout
# (translate along fold axis to zero, then reverse along fold axis). Aggregate
# dots in a particular location using logical OR.

# How to transform coordinates on moving half?
# Just maintain list of dots, and use set operations to aggregate, specifically
# set union. This will handle the logical OR.
# Fold up, and fold left.

# If fold: y=7, then new_y = -old_y + (2y), x remains the same.
# -8 + 14 = 6, -14 + 14 = 0
# Similar,y for folds on x

import re
from more_itertools import partition

input = """6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"""
input = open("13/input.txt", "r").read()


def parse_fold(fold_str):
  match = re.search("fold along ([xy])=([0-9]+)", fold_str)
  if not match:
    raise Exception(f"Unable to parse fold: {fold_str}")
  return match.group(1), int(match.group(2))


def coordinate_dim_for_fold(fold):
  coordinate, fold_loc = fold
  assert (coordinate in set(["x", "y"]))
  coordinate_dim = 0 if coordinate == "x" else 1
  return coordinate_dim


def transform_moving_dot(dot, fold):
  coordinate_dim = coordinate_dim_for_fold(fold)
  _, fold_loc = fold
  assert (fold_loc < dot[coordinate_dim])

  transformed_dot = list(dot)
  transformed_dot[
      coordinate_dim] = -transformed_dot[coordinate_dim] + 2 * fold_loc

  return tuple(transformed_dot)


def perform_fold(dots, fold):
  coordinate_dim = coordinate_dim_for_fold(fold)
  _, fold_loc = fold

  stationary_half, moving_half = partition(
      lambda dot: dot[coordinate_dim] > fold_loc, dots)

  transformed_moving_half = set(
      [transform_moving_dot(dot, fold) for dot in moving_half])

  new_dots = transformed_moving_half.union(stationary_half)
  return new_dots


def perform_folds(dots, folds):
  for fold in folds:
    dots = perform_fold(dots, fold)
  return dots


dots, folds = [x.splitlines() for x in input.split("\n\n")]
folds = [parse_fold(x) for x in folds]
dots = set([tuple([int(i) for i in x.split(",")]) for x in dots])

print(len(perform_fold(dots, folds[0])))
