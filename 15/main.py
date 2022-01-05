#! python

import sys, os
import heapq
import numpy as np

sys.path.insert(1, os.path.abspath("."))

from utils import read_matrix_from_str, taxi_cab_neighborhood

input = """1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"""
input = open("15/input.txt", "r").read()

risk_map = read_matrix_from_str(input)


def get_neighbors_and_risks(idx, risk_map):
  neighbor_idxs = list(taxi_cab_neighborhood(idx, risk_map.shape))
  rows, cols = zip(*neighbor_idxs)
  risks = risk_map[rows, cols]
  return zip(neighbor_idxs, risks)


def find_lowest_risk_path(risk_map):
  start_position = (0, 0)
  end_position = (risk_map.shape[0] - 1, risk_map.shape[1] - 1)

  visited_nodes = set([start_position])
  boundary = [(0, start_position)]
  heapq.heapify(boundary)

  while len(boundary) > 0:
    path_cost, idx = heapq.heappop(boundary)

    if idx == end_position:
      return path_cost

    for neighbor_idx, neighbor_risk in get_neighbors_and_risks(idx, risk_map):
      if neighbor_idx not in visited_nodes:
        visited_nodes.add(neighbor_idx)
        heapq.heappush(boundary, (path_cost + neighbor_risk, neighbor_idx))

  raise Exception("Unable to find a path from start position to end position")


TILES = (5, 5)
MAX_RISK = 9


def tile_risk_map(risk_map):
  tiled = np.tile(risk_map, TILES)
  tile_rows = np.vsplit(tiled, TILES[0])
  tile_cells = [np.hsplit(tr, TILES[1]) for tr in tile_rows]

  for i, tile_cell_row in enumerate(tile_cells):
    for j, cell in enumerate(tile_cell_row):
      cell[:] = (cell + i + j)
      cell[cell > MAX_RISK] = cell[cell > MAX_RISK] - MAX_RISK

  return tiled


print(find_lowest_risk_path(tile_risk_map(risk_map)))
