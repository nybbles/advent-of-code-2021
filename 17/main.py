#! python

import re

input = "target area: x=20..30, y=-10..-5"

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
  return ((int(match.group(1)), int(match.group(2))), (int(match.group(3)),
                                                       int(match.group(4))))


print(parse_input(input))