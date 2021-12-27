#! python

from more_itertools import bucket

input = """be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"""

input = open("08/input.txt", 'r').read()


def parse_signal_patterns(line):
  return [pattern.strip() for pattern in line.strip().split(" ")]


input = [(set(parse_signal_patterns(unique_signal_patterns)),
          parse_signal_patterns(output_values))
         for unique_signal_patterns, output_values in
         [line.strip().split("|") for line in input.splitlines()]]

# Digits 1, 4, 7 and 8 use a unique number of segments each. "1" uses 2
# segments, "4" uses 4 segments, "8" uses all 7 segments and "7" uses 3
unique_segment_lengths = set([2, 4, 7, 3])

segment_length_to_digits = {
    # Unique segment lengths
    2: set([1]),
    4: set([4]),
    7: set([8]),
    3: set([7]),

    # Non-unique segment lengths
    6: set([0, 6, 9]),
    5: set([2, 3, 5]),
}


def patterns_by_segment_length(segment_patterns):
  bucketed = bucket(segment_patterns, key=lambda x: len(x))
  return {length: set(bucketed[length]) for length in list(bucketed)}


def segment_to_digit_mapping(by_segment_lengths):
  mapping = {
      digit: set(list(patterns)[0])
      for digit, patterns in {
          1: by_segment_lengths[2],
          4: by_segment_lengths[4],
          7: by_segment_lengths[3],
          8: by_segment_lengths[7],
      }.items()
  }

  # Number "1" only uses right-side segments.
  # Number "4" is "1" plus middle segment and top-left segment.
  # Number "7" is "1" plus top-middle segment.

  # Disambiguation for six segments:
  # Numbers "4" plus "7" is a strict subset of number "9".
  union_4_7 = mapping[4].union(mapping[7])
  mapping[9] = set([
      segment for segment in by_segment_lengths[6]
      if set(segment).issuperset(union_4_7)
  ][0])

  # Number "6" has exactly one of the segments in "1" missing.
  mapping[6] = set([
      segment for segment in by_segment_lengths[6]
      if len(mapping[1].difference(set(segment))) == 1
  ][0])

  # Therefore, "0"  is the remaining six segment digit.
  mapping[0] = set([
      segment for segment in by_segment_lengths[6]
      if not (set(segment).issuperset(union_4_7)
              or len(mapping[1].difference(set(segment))) == 1)
  ][0])

  # Disambiguation for five segments
  # Number "5" has one segment removed from "6"
  mapping[5] = set([
      segment for segment in by_segment_lengths[5]
      if set(mapping[6]).issuperset(segment)
  ][0])

  # Number "3" is a super-set of "1", and "2" and "5" are not.
  mapping[3] = set([
      segment for segment in by_segment_lengths[5]
      if set(segment).issuperset(mapping[1])
  ][0])

  # Therefore, "2" is the remaining five segment digit.
  mapping[2] = set([
      segment for segment in by_segment_lengths[5]
      if not (set(mapping[6]).issuperset(segment)
              or set(segment).issuperset(mapping[1]))
  ][0])

  # invert mapping to get segment to digit
  inverted_mapping = {
      "".join(sorted(list(value))): key
      for key, value in mapping.items()
  }

  return inverted_mapping


def decode_number(segment_to_digit, segments):
  digits = [
      segment_to_digit[segment] for segment in
      ["".join(sorted(list(segments))) for segments in segments]
  ]
  number = int("".join([str(digit) for digit in digits]))
  return number


def decode_all_numbers(input):
  for signal_patterns, segments in input:
    segment_to_digit = segment_to_digit_mapping(
        patterns_by_segment_length(signal_patterns))
    yield decode_number(segment_to_digit, segments)


ans = sum(decode_all_numbers(input))
print(ans)
