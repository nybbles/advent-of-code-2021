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
  def __init__(self, stack_state):
    self.stack_state = stack_state


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
    raise IncompleteLineException(end_stack)


def stack_state_to_expected_closing_delimiters(stack_state):
  result = [opening_delimiters[char][0] for char in stack_state]
  result.reverse()
  return result


def score_incomplete_stack_state(stack_state):
  closing_delimiter_to_score = {')': 1, ']': 2, '}': 3, '>': 4}

  def update_score(score, next_char):
    return score * 5 + closing_delimiter_to_score[next_char]

  return reduce(update_score,
                stack_state_to_expected_closing_delimiters(stack_state), 0)


def check_lines(lines):
  syntax_error_score = 0
  incomplete_stack_scores = []
  for line in lines:
    try:
      check_line(line)
    except CorruptedLineException as err:
      _, score = closing_delimiters[err.found]
      syntax_error_score = syntax_error_score + score
      continue
    except IncompleteLineException as err:
      incomplete_stack_scores.append(
          score_incomplete_stack_state(err.stack_state))
      continue

  incomplete_stack_scores.sort()
  mid_score = incomplete_stack_scores[int(len(incomplete_stack_scores) / 2)]

  return syntax_error_score, mid_score


print(check_lines(input))