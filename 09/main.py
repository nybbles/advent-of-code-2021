#! python

import numpy as np
from functools import reduce

input = """2199943210
3987894921
9856789892
8767896789
9899965678"""

input = open("09/input.txt", "r").read()


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


def basin_for_low_point(height_map, low_point):
  low_point_idx, low_point_height, _ = low_point

  # Expand from low point to determine basin.
  # Basin boundary entry: (idx, previous idx height)
  basin = set([low_point_idx])

  def get_basin_boundary_update(candidate_idx, candidate_height):
    return [(idx, candidate_height)
            for idx in neighborhood(candidate_idx, height_map.shape)
            if idx not in basin]

  basin_boundary = get_basin_boundary_update(low_point_idx, low_point_height)

  while len(basin_boundary) > 0:
    candidate_idx, prev_idx_height = basin_boundary.pop()
    candidate_height = height_map[candidate_idx]
    if (candidate_height >= 9 or candidate_height <= prev_idx_height):
      continue

    basin.add(candidate_idx)
    basin_boundary.extend(
        get_basin_boundary_update(candidate_idx, candidate_height))

  return basin


def risk_level(low_points):
  return sum([x + 1 for x in low_points])


low_points = list(find_low_points(height_map))

ans = risk_level([height for _, height, _ in low_points])
print(ans)

basins = [
    basin_for_low_point(height_map, low_point) for low_point in low_points
]
basins.sort(key=lambda x: len(x), reverse=True)

ans = reduce(lambda x, y: x * y, [len(basin) for basin in basins[0:3]], 1)
print(ans)