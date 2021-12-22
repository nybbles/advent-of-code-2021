#! python

from itertools import chain
from more_itertools import ilen

school = [3, 4, 3, 1, 2]
school = [int(x.strip()) for x in open("06/input.txt", 'r').read().split(",")]


def simulate_one_day_one_fish(fish):
  if fish > 0:
    return [fish - 1]
  else:
    return [6, 8]


def simulate_one_day(school):
  return chain(*[simulate_one_day_one_fish(fish) for fish in school])


def simulate_days(school, ndays):
  for _ in range(ndays):
    school = simulate_one_day(school)
  return school


print(ilen(simulate_days(school, 80)))
