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


most_common_bits = get_most_common_bits(data)
gamma_rate = bits_to_number(most_common_bits)
epsilon_rate = bits_to_number(flip_bits(most_common_bits))

print(f"gamma rate: {gamma_rate}")
print(f"epsilon rate: {epsilon_rate}")