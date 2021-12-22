#! python

import numpy as np
from collections import defaultdict
from itertools import chain
from more_itertools import take

input = [
    "0,9 -> 5,9",
    "8,0 -> 0,8",
    "9,4 -> 3,4",
    "2,2 -> 2,1",
    "7,0 -> 7,4",
    "6,4 -> 2,0",
    "0,9 -> 2,9",
    "3,4 -> 1,4",
    "0,0 -> 8,8",
    "5,5 -> 8,2",
]
input = open('05/input.txt', 'r').readlines()

Point = (int, int)
Line = (Point, Point)


def parse_line(input):
  p0, p1 = [
      tuple([int(y) for y in x.strip().split(",")]) for x in input.split("->")
  ]
  return (p0, p1)


def is_line_axis_aligned(line: Line) -> bool:
  p0, p1 = line
  return p0[0] == p1[0] or p0[1] == p1[1]


def expand_line_to_points(line: Line):
  # Line is expected to be axis aligned
  assert (is_line_axis_aligned(line))
  p0, p1 = line
  changing_dim = 0 if p0[0] != p1[0] else 1
  unchanging_dim = 1 if changing_dim == 0 else 0

  extent = (
      min([p0[changing_dim], p1[changing_dim]]),
      max([p0[changing_dim], p1[changing_dim]]),
  )

  for x in range(extent[0], extent[1] + 1):
    new_point = [-1, -1]
    new_point[unchanging_dim] = p0[unchanging_dim]
    new_point[changing_dim] = x
    yield tuple(new_point)


lines = [parse_line(x) for x in input]

axis_aligned_lines = [x for x in lines if is_line_axis_aligned(x)]

expanded_points = chain(
    *[expand_line_to_points(x) for x in axis_aligned_lines])

point_cloud_densities = defaultdict(int)
for point in expanded_points:
  point_cloud_densities[point] = point_cloud_densities[point] + 1

dangerous_points = [
    point for point, count in point_cloud_densities.items() if count >= 2
]

print(len(dangerous_points))
