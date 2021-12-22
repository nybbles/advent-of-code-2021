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


def expand_line_to_points(line: Line):
  # Line is expected to be axis aligned or diagonal
  p0, p1 = line

  direction = np.sign(np.array(p1) - np.array(p0))
  p = p0

  while True:
    yield p

    if (p == p1):
      break

    p = tuple(np.array(p) + direction)


lines = [parse_line(x) for x in input]

expanded_points = chain(*[expand_line_to_points(x) for x in lines])

point_cloud_densities = defaultdict(int)
for point in expanded_points:
  point_cloud_densities[point] = point_cloud_densities[point] + 1

dangerous_points = [
    point for point, count in point_cloud_densities.items() if count >= 2
]

print(len(dangerous_points))
