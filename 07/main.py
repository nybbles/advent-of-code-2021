#! python

crabs = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
crabs = [int(x.strip()) for x in open("07/input.txt", 'r').read().split(",")]


def cost(crab, target):
  return abs(crab - target)


def position_costs(crabs):
  for target in range(min(crabs), max(crabs) + 1):
    yield (target, sum([cost(crab, target) for crab in crabs]))


def cheapest_position(crabs):
  return sorted(list(position_costs(crabs)), key=lambda x: x[1])[0]


target_position, target_cost = cheapest_position(crabs)
print(f"target position: {target_position}")
print(f"target cost: {target_cost}")