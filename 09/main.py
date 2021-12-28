#! python

from more_itertools import stagger, islice_extended
from itertools import chain

input = """2199943210
3987894921
9856789892
8767896789
9899965678""".splitlines()
input = open("09/input.txt", "r").readlines()
input = [[int(x) for x in list(line.strip())] for line in input]

WINDOW_SIZE = 3


def inclusive_window(xs, offsets=[-1, 0, 1]):
  return islice_extended(stagger(xs, longest=True, offsets=offsets), 0, -1)


def neighborhoods(windowed_row):
  assert (len(windowed_row) == WINDOW_SIZE)

  points, same_row_neighbors = zip(*[(point,
                                      [n for n in neighbors if n is not None])
                                     for point, *neighbors in inclusive_window(
                                         windowed_row[1], offsets=[0, -1, 1])])

  other_row_neighbors = zip(
      *[windowed_row[idx] for idx in [0, 2] if windowed_row[idx] is not None])
  other_row_neighbors = [list(x) for x in other_row_neighbors
                         ]  # convert from tuples to lists

  all_neighbors = [
      x + y for x, y in zip(same_row_neighbors, other_row_neighbors)
  ]

  points_with_neighborhoods = zip(points, all_neighbors)

  return points_with_neighborhoods


def is_low_point(point, neighborhood):
  return point < min(neighborhood)


def risk_level(low_points):
  return sum([x + 1 for x in low_points])


windowed_rows = inclusive_window(input)
points_with_neighborhoods = chain(
    *[neighborhoods(windowed_row) for windowed_row in windowed_rows])

low_points = [(point, neighborhood)
              for point, neighborhood in points_with_neighborhoods
              if is_low_point(point, neighborhood)]

ans = risk_level([point for point, _ in low_points])
print(ans)