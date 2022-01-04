#! python

import re
from more_itertools import pairwise, bucket, ilen
from functools import reduce

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


def polymer_to_polymer_pair_hist(polymer):
  bucketed = bucket(pairwise(polymer), key=lambda x: x)
  return {x: ilen(bucketed[x]) for x in list(bucketed)}


def grow_polymer_one_step(polymer_pair_hist, pair_insertion_rules):
  result = {}
  for pair in polymer_pair_hist.keys():
    if pair in pair_insertion_rules:
      element = pair_insertion_rules[pair]
      new_pairs = [(pair[0], element), (element, pair[1])]
      for new_pair in new_pairs:
        result[new_pair] = polymer_pair_hist[pair] + (
            0 if new_pair not in result else result[new_pair])
    else:
      result[pair] = polymer_pair_hist[pair]

  return result


def grow_polymer_n_steps(n, polymer_pair_hist, pair_insertion_rules):
  for i in range(n):
    print(f"Step {i}")
    polymer_pair_hist = grow_polymer_one_step(polymer_pair_hist,
                                              pair_insertion_rules)
  return polymer_pair_hist


def polymer_pair_hist_to_element_counts(polymer_template, polymer_pair_hist):
  # How to account for double counting?
  # Only the start and end element are not double-counted. Add 1 to both of
  # those element counts and then divide everything by two.

  border_elements = set([polymer_template[i] for i in [0, -1]])

  def adjust_for_double_counting(element, count):
    # if the border has the same element, then the adjustment has to be aggregated
    adjustment_for_border_element = 1 if len(border_elements) == 2 else 2
    return int(
        (count +
         (adjustment_for_border_element if element in border_elements else 0))
        / 2)

  bucketed = bucket([
      item for sublist in [[(x, count) for x in pair]
                           for pair, count in polymer_pair_hist.items()]
      for item in sublist
  ],
                    key=lambda x: x[0])

  counts = {
      element: adjust_for_double_counting(
          element,
          reduce(lambda x, y: x + y,
                 [count for _, count in bucketed[element]]))
      for element in list(bucketed)
  }

  return counts


def get_most_least_common_elements(polymer_template, polymer_pair_hist):
  counts = list(
      polymer_pair_hist_to_element_counts(polymer_template,
                                          polymer_pair_hist).items())
  counts.sort(key=lambda x: x[1])
  print(counts)

  most_common = counts[-1]
  least_common = counts[0]
  return (most_common, least_common)


polymer_template, pair_insertion_rules = input.split("\n\n")
polymer_template = polymer_template.strip()
polymer_pair_hist = polymer_to_polymer_pair_hist(polymer_template)
pair_insertion_rules = {
    tuple(pair): element
    for pair, element in
    [parse_pair_insertion_rule(x) for x in pair_insertion_rules.splitlines()]
}

result_polymer_pair_hist = grow_polymer_n_steps(40, polymer_pair_hist,
                                                pair_insertion_rules)

most_common, least_common = get_most_least_common_elements(
    polymer_template, result_polymer_pair_hist)
print(most_common[1] - least_common[1])
