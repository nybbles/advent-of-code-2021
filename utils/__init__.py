import numpy as np
from functools import reduce


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


def threadf(x, fs):
  return reduce(lambda r, f: f(r), fs, x)


def taxi_cab_neighborhood(idx, shape):
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
