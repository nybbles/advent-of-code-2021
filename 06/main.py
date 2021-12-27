#! python

from more_itertools import ilen, bucket
from collections import defaultdict

# school = [3, 4, 3, 1, 2]
school = [int(x.strip()) for x in open("06/input.txt", 'r').read().split(",")]


def school_histogram(school):
  bucketed = bucket(school, key=lambda x: x)
  return {x: ilen(bucketed[x]) for x in list(bucketed)}


def simulate_one_day(school_histogram):
  next_day_histogram = defaultdict(int)
  for age, number in school_histogram.items():
    if age > 0:
      next_day_histogram[age - 1] = next_day_histogram[age - 1] + number
    else:
      next_day_histogram[8] = next_day_histogram[8] + number
      next_day_histogram[6] = next_day_histogram[6] + number

  return next_day_histogram


def simulate_days(school_histogram, ndays):
  for i in range(ndays):
    print(f"simulating day {i}")
    school_histogram = simulate_one_day(school_histogram)
  return school_histogram


def count_fish_in_histogram(school_histogram):
  return sum(school_histogram.values())


print(count_fish_in_histogram(simulate_days(school_histogram(school), 256)))
