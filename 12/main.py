#! python

from functools import reduce
from more_itertools import ilen

# input is edge list, represent graph as adjacency list
# assumption: the graph does not contain cycles made up of consecutive large
# caves - this would mean an infinite number of paths (since you can just go any
# number of times through the cycle of large caves).

input = """start-A
start-b
A-c
A-b
b-d
A-end
b-end"""
input = open("12/input.txt", "r").read()


def parse_edge_list(input):
  for line in input.splitlines():
    node0, node1 = line.split("-")
    yield (node0, node1)
    yield (node1, node0)


def acc_adj_list(adj_list, edge):
  node0, node1 = edge
  if node0 not in adj_list:
    adj_list[node0] = set()
  adj_list[node0].add(node1)
  return adj_list


graph = reduce(acc_adj_list, parse_edge_list(input), {})


def is_small_cave(node):
  return node.islower()


def find_all_paths(graph):
  # boundary entry: (next node, path)
  # never add start to boundary. path complete when end is the node
  boundary = [("start", ["start"])]

  while len(boundary) > 0:
    node, path = boundary.pop()
    if node == "end":
      yield path
      continue

    visited_small_caves = set([c for c in path if is_small_cave(c)])
    neighbors = [x for x in graph[node] if x != "start"]
    for neighbor in neighbors:
      if is_small_cave(neighbor) and neighbor in visited_small_caves:
        continue

      new_path = path.copy()
      new_path.append(neighbor)
      boundary.append((neighbor, new_path))


print(graph)
print(ilen(find_all_paths(graph)))
