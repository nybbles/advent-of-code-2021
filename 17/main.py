#! python

import re
from itertools import count

input = "target area: x=20..30, y=-10..-5"
input = open("17/input.txt", "r").read()

# Split the problem up into x and y coordinates separately.
# x coordinate: x = (dx (dx + 1)) / 2
# At each x within the target, the trajectory that maximizes y is the one that
# has smallest dx. If there were a trajectory with larger dx, then y could have
# been made higher by aiming the probe higher.
# Compute smallest dx for each x in the target area. This is independent of y.
# There will be some set of initial dx where the probe ends up in the target
# area and is slow (in fact, possibly zero somewhere).
# Then for each x in the target area, for its smallest dx, find the highest y
# trajectory. I'm not sure how to do that though.
# Each x in the target area has some number of time steps it takes to get there
# (where we are minimizing the final dx, as above). The number of time steps can
# be used to solve for y. Need to find some initial dy such that after that
# number of time steps, the final y is within the target.
# y = (dy * n) - (n * (n-1)) / 2


def parse_input(input):
  match = re.search(
      "target area: x=([-0-9]+)..([-0-9]+), y=([-0-9]+)..([-0-9]+)", input)
  if not match:
    raise Exception("Unable to parse input")
  return (int(match.group(1)), int(match.group(2))), (int(match.group(3)),
                                                      int(match.group(4)))


def cumsum_iter(start):
  acc = 0
  for i in count(start):
    acc = i + acc
    yield acc


def cumsum(n):
  return int(n * (n + 1) / 2)


def determine_dx(target_x):
  x0, x1 = target_x
  initial_dx = None
  x_stalled = None
  result = []

  for i, x in enumerate(cumsum_iter(1)):
    if x >= x0:
      initial_dx = i + 1
      x_stalled = x
      break

  for x in range(x0, min(x1, x_stalled) + 1):
    dx = x_stalled - x
    result.append((x, initial_dx, dx))

  if x1 <= x_stalled:
    return result

  final_x = x_stalled

  while True:
    assert (final_x < x1)

    x_stalled = final_x + 1
    initial_dx = initial_dx + 1
    final_x = cumsum(initial_dx)

    for x in range(x_stalled, min(x1, final_x) + 1):
      dx = final_x - x
      result.append((x, initial_dx, dx))

    if x1 <= final_x:
      return result


def get_ntimesteps(initial_dx, dx):
  # The number of timesteps passed can be computed from the starting speed and
  # the ending speed, since speed decreases by 1 on each timestep.
  return initial_dx - dx


def dy_to_maxy(dy):
  if dy <= 0:
    return dy
  return cumsum(dy)


def determine_maxy_trajectory(initial_dx, dx, target_y):
  # TODO: Need special handling for dx = 0, because they can use more timesteps
  # y = (dy * n) - (n * (n-1)) / 2
  # y = dy * n - constant0
  # y + constant0 = dy * constant1
  # dy = (y + constant0) / constant1
  # if dx = 0, then timesteps >= n.
  # Assumption: Trajectories where dx=0, but dy>0 are not valid.
  # cumsum(n) to get to the apex, then cumsum(m) to descend, where m>=n

  y1, y0 = target_y
  assert (y0 >= y1)
  n = get_ntimesteps(initial_dx, dx)

  if (dx == 0):
    print(initial_dx, dx, n)
    for initial_dy in range(n, 1, -1):
      max_y = cumsum(initial_dy)
      for i in cumsum_iter(1):
        final_y = max_y - i
        print(final_y, y0, y1)
        if final_y < y1:
          break
        if final_y <= y0:
          print("Found dx=0 trajectory", final_y, initial_dy, max_y)
          return final_y, initial_dy, max_y

    return None
  else:
    dys = []

    for y in range(y1, y0 + 1):
      dy = (y + cumsum(n - 1)) / n
      if not dy.is_integer():
        continue
      dys.append((y, int(dy)))

    if len(dys) == 0:
      return None

    best_dy = max(dys, key=lambda x: x[1])
    return best_dy[0], best_dy[1], dy_to_maxy(best_dy[1])


target_x, target_y = parse_input(input)
print(target_x, target_y)

on_target_trajectories = []
for trajectory in determine_dx(target_x):
  x, initial_dx, dx = trajectory
  result = determine_maxy_trajectory(initial_dx, dx, target_y)
  if result is not None:
    y, initial_dy, max_y = result
    on_target_trajectories.append((initial_dx, initial_dy, x, y, max_y))

on_target_trajectories.sort(key=lambda x: x[-1], reverse=True)
print(on_target_trajectories[0])
