#! python

import numpy as np

from more_itertools import stagger, islice_extended
from itertools import chain

input = """2199943210
3987894921
9856789892
8767896789
9899965678"""

# input = open("09/input.txt", "r").read()


def get_array_shape(input):
  rows = input.splitlines()
  return [len(rows), len(rows[0])]


height_map_str = " ".join([
    " ".join([c for c in line.strip()]) for line in input.strip().splitlines()
])

height_map_shape = get_array_shape(input)
height_map = np.fromstring(height_map_str, dtype=int, sep=" ")
height_map = np.reshape(height_map, height_map_shape)


def neighborhood(idx, shape):
  x, y = idx
  x_lim, y_lim = shape

  assert (x >= 0 and x < x_lim)
  assert (y >= 0 and y < y_lim)

  if (x >= 1):
    yield (x - 1, y)

  if (x + 1 < x_lim):
    yield (x + 1, y)

  if (y >= 1):
    yield (x, y - 1)

  if (y + 1 < y_lim):
    yield (x, y + 1)


def idxs_to_values(mat2d, idxs):
  rows, cols = zip(*idxs)
  return mat2d[np.array(rows), np.array(cols)]


def is_low_point(point_val, neighbor_vals):
  return point_val < min(neighbor_vals)


def find_low_points(height_map):
  for idx, x in np.ndenumerate(height_map):
    neighbor_idxs = list(neighborhood(idx, height_map.shape))
    neighbor_vals = idxs_to_values(height_map, neighbor_idxs)

    if (is_low_point(x, neighbor_vals)):
      yield (idx, x, neighbor_vals)


def risk_level(low_points):
  return sum([x + 1 for x in low_points])


ans = risk_level([height for _, height, _ in find_low_points(height_map)])
print(ans)