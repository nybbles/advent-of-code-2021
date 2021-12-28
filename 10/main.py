#! python

from functools import reduce

input = """[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"""

input = [[c for c in line] for line in input.splitlines()]
input = open("10/input.txt", "r").readlines()

closing_delimiters = {
    ')': ('(', 3),
    ']': ('[', 57),
    '}': ('{', 1197),
    '>': ('<', 25137),
}

opening_delimiters = {
    opening: (closing, score)
    for closing, (opening, score) in closing_delimiters.items()
}


class CorruptedLineException(Exception):
  def __init__(self, expected, found):
    self.expected = expected
    self.found = found


class IncompleteLineException(Exception):
  def __init__(self):
    pass


def check_line(line):
  def check_next_char(stack, next_char):
    if next_char in opening_delimiters:
      stack.append(next_char)
    elif next_char in closing_delimiters:
      expected = stack.pop()
      next_char_closing, _ = closing_delimiters[next_char]

      if (next_char_closing != expected):
        expected_closing, _ = opening_delimiters[expected]
        raise CorruptedLineException(expected_closing, next_char)

    return stack

  end_stack = reduce(check_next_char, line, [])

  if (len(end_stack) != 0):
    raise IncompleteLineException()


def check_lines(lines):
  syntax_error_score = 0
  for line in lines:
    try:
      check_line(line)
    except CorruptedLineException as err:
      _, score = closing_delimiters[err.found]
      syntax_error_score = syntax_error_score + score
      continue
    except IncompleteLineException:
      continue

  return syntax_error_score


print(check_lines(input))