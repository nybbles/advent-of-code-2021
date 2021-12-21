#! python

import numpy as np
import math

input = open('03/input.txt', 'r').readlines()

matrix_fmt_input = "; ".join([" ".join([char for char in x])
                              for x in input]).strip()
data = np.array(np.matrix(
    matrix_fmt_input,
    dtype=int,
))


def get_most_common_bits(data):
  ndatapoints = data.shape[0]
  majority_threshold = math.ceil(ndatapoints / 2)
  column_sums = np.sum(data, axis=0)
  mask = column_sums >= majority_threshold
  return np.asarray(mask, dtype=int)


def flip_bits(bits):
  mask = bits == 0
  return np.asarray(mask, dtype=int)


def bits_to_number(bits):
  number = 0
  for i, bit in enumerate(reversed(list(np.nditer(bits)))):
    if (bit == 1):
      number += int(math.pow(2, i))

  return number


def search_for_rating(data, most_common_value_fn):
  nbits = data.shape[1]
  subrange = data

  for i in range(nbits):
    col = subrange[:, i]
    ones = np.sum(np.asarray((col == 1), dtype=int))
    zeros = np.sum(np.asarray((col == 0), dtype=int))
    most_common_value = most_common_value_fn(zeros, ones)
    subrange = subrange[subrange[:, i] == most_common_value]

    assert (subrange.shape[0] >= 1)

    if subrange.shape[0] == 1:
      break

  return subrange[0]


def find_oxygen_generator_rating(data):
  def most_common_value(zeros, ones):
    return 1 if ones >= zeros else 0

  return bits_to_number(search_for_rating(data, most_common_value))


def find_co2_scrubber_rating(data):
  def most_common_value(zeros, ones):
    return 0 if zeros <= ones else 1

  return bits_to_number(search_for_rating(data, most_common_value))


most_common_bits = get_most_common_bits(data)
gamma_rate = bits_to_number(most_common_bits)
epsilon_rate = bits_to_number(flip_bits(most_common_bits))

oxygen_generator_rating = find_oxygen_generator_rating(data)
co2_scrubber_rating = find_co2_scrubber_rating(data)

print(f"gamma rate: {gamma_rate}")
print(f"epsilon rate: {epsilon_rate}")

print(f"oxygen generator rating: {oxygen_generator_rating}")
print(f"co2 scrubber rating: {co2_scrubber_rating}")