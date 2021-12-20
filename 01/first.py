#! python

from more_itertools import pairwise, windowed

# Read input from file
data = [int(x) for x in open('01/input.txt', 'r').readlines()]

# Compute increases
sliding_sums = [sum(xs) for xs in windowed(data, 3)]
nincreases = len([(x0, x1) for x0, x1 in pairwise(sliding_sums) if x1 > x0])

print(nincreases)
