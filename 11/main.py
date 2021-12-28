#! python

import numpy as np
from itertools import chain
from more_itertools import bucket, ilen

input = """5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"""
input = open("11/input.txt", "r").read()


def get_matrix_shape_from_str(input):
  rows = input.splitlines()
  return [len(rows), len(rows[0])]


def read_matrix_from_str(input):
  matrix_str = " ".join([
      " ".join([c for c in line.strip()])
      for line in input.strip().splitlines()
  ])
  matrix_shape = get_matrix_shape_from_str(input)
  return np.reshape(np.fromstring(matrix_str, dtype=int, sep=" "),
                    matrix_shape)


octopus_map = read_matrix_from_str(input)


def neighborhood(idx, map_shape):
  x, y = idx
  x_lim, y_lim = map_shape

  x_vals = range(max(0, x - 1), min(x_lim, x + 2))
  y_vals = range(max(0, y - 1), min(y_lim, y + 2))

  for neighbor_x in x_vals:
    for neighbor_y in y_vals:
      neighbor_idx = (neighbor_x, neighbor_y)
      if neighbor_idx == tuple(idx):
        continue
      yield neighbor_idx


def histogram(xs):
  bucketed = bucket(xs, key=lambda x: x)
  return {x: ilen(bucketed[x]) for x in list(bucketed)}


def idx_histogram_to_update(hist):
  idxs = hist.keys()
  update = list(hist.values())
  rows, cols = zip(*idxs)
  return rows, cols, update


def propagate_flash(octopus_map):
  flash_idxs = list(np.argwhere(octopus_map > 9))
  nflashes_before_propagate = len(flash_idxs)

  if (nflashes_before_propagate == 0):
    # No flashes, so nothing to propagate
    return octopus_map, nflashes_before_propagate

  octopus_map[octopus_map > 9] = 0
  flashed_idxs = set([tuple(idx) for idx in np.argwhere(octopus_map == 0)])

  neighbor_idxs = [
      idx for idx in chain(*[
          neighborhood(flash_idx, octopus_map.shape)
          for flash_idx in flash_idxs
      ]) if idx not in flashed_idxs
  ]

  if (len(neighbor_idxs) == 0):
    # no neighbors to propagate to
    return octopus_map, nflashes_before_propagate

  neighbor_idx_hist = histogram(neighbor_idxs)
  rows, cols, update = idx_histogram_to_update(neighbor_idx_hist)

  octopus_map[rows, cols] = octopus_map[rows, cols] + update

  return octopus_map, nflashes_before_propagate


def simulate_one_step(octopus_map):
  octopus_map = octopus_map + 1
  total_flashes = 0

  while True:
    octopus_map, nflashes_before_propagate = propagate_flash(octopus_map)
    total_flashes = total_flashes + nflashes_before_propagate

    if (nflashes_before_propagate == 0):
      break

  return octopus_map, total_flashes


def simulate_n_steps(octopus_map, n):
  total_flashes = 0
  for _ in range(n):
    octopus_map, flashes = simulate_one_step(octopus_map)
    total_flashes = total_flashes + flashes

  return octopus_map, total_flashes


print("Before:")
print(octopus_map)

print("\nAfter:")
print(simulate_n_steps(octopus_map, 100))