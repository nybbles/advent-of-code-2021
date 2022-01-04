#! python

import re
from more_itertools import pairwise, bucket, ilen

input = """NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"""

input = open("14/input.txt", "r").read()


def parse_pair_insertion_rule(input):
  match = re.search("([A-Z]{2}) -> ([A-Z])", input)
  if not match:
    raise Exception(f"Could not parse string for pair insertion rule: {input}")
  pair = match.group(1)
  element = match.group(2)
  return (pair, element)


polymer_template, pair_insertion_rules = input.split("\n\n")
polymer_template = polymer_template.strip()
pair_insertion_rules = {
    tuple(pair): element
    for pair, element in
    [parse_pair_insertion_rule(x) for x in pair_insertion_rules.splitlines()]
}


def grow_polymer_one_step(polymer, pair_insertion_rules):
  result = [polymer[0]]
  pairs = pairwise(polymer)
  for pair in pairs:
    new_section = list(pair[1])
    if pair in pair_insertion_rules:
      element = pair_insertion_rules[pair]
      new_section.insert(0, element)
    result.extend(new_section)
  return "".join(result)


def grow_polymer_n_steps(n, polymer, pair_insertion_rules):
  for _ in range(n):
    polymer = grow_polymer_one_step(polymer, pair_insertion_rules)
  return polymer


def get_most_least_common_elements(polymer):
  bucketed = bucket(polymer, key=lambda x: x)
  counts = sorted([(x, ilen(bucketed[x])) for x in list(bucketed)],
                  key=lambda x: x[1])
  most_common = counts[-1]
  least_common = counts[0]
  return (most_common, least_common)


result_polymer = grow_polymer_n_steps(10, polymer_template,
                                      pair_insertion_rules)

most_common, least_common = get_most_least_common_elements(result_polymer)
print(most_common[1] - least_common[1])
