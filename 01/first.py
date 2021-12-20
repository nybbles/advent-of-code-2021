#! python

from more_itertools import pairwise

# Read input from file
data = [int(x) for x in open('01/input.txt', 'r').readlines()]

# Compute increases
nincreases = len([(x0, x1) for x0, x1 in pairwise(data) if x1 > x0])
print(nincreases)
