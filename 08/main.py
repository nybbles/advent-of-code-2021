#! python

from itertools import chain

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

output_values = list(zip(*input))[1]


def flatten_one_level(xss):
  for xs in xss:
    for x in xs:
      yield x


ans = [
    output_value for output_value in flatten_one_level(output_values)
    if len(output_value) in unique_segment_lengths
]

print(len(ans))