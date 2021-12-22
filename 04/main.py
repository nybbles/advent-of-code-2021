#! python

from itertools import dropwhile, islice
from more_itertools import first, take, chunked, filter_except, split_at
import numpy as np

input = """7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"""

input = open('04/input.txt', 'r').read()

# Parse input seq and boards
input_lines = input.splitlines()
input_seq = [int(x) for x in take(1, input_lines)[0].split(",")]

BOARD_HEIGHT = 5
unparsed_boards = split_at(islice(input_lines, 2, None), lambda x: len(x) == 0)

# Board representation and operations


class Board:
  EMPTY_CELL_VALUE = -1

  def __init__(self, board_lines):
    assert (len(board_lines) == BOARD_HEIGHT)
    self.board_state = np.array(np.matrix("; ".join(board_lines), dtype=int))

  def mark_number_and_check(self, number):
    self.board_state[self.board_state == number] = self.EMPTY_CELL_VALUE
    check_matrix = self.board_state == self.EMPTY_CELL_VALUE

    complete_columns_exist = np.any(np.all(check_matrix, axis=0))
    complete_rows_exist = np.any(np.all(check_matrix, axis=1))

    board_was_completed = complete_columns_exist or complete_rows_exist
    return board_was_completed

  def compute_board_score(self):
    return np.sum(self.board_state[self.board_state != self.EMPTY_CELL_VALUE])


boards = [Board(board_lines) for board_lines in unparsed_boards]

completed_boards = []
number_on_board_completion = None

for x in input_seq:
  for board in boards:
    board_was_completed = board.mark_number_and_check(x)
    if board_was_completed:
      completed_boards.append(board)

  if len(completed_boards) == 0:
    continue

  ncompleted_boards = len(completed_boards)
  print(f"Completed {ncompleted_boards} boards on number {x}")

  completed_boards.sort(key=lambda x: x.compute_board_score(), reverse=True)
  number_on_board_completion = x

  break

if (completed_boards is None):
  print("No boards were completed")
else:
  best_board = completed_boards[0]
  best_board_score = best_board.compute_board_score()

  print("Boards were completed")
  print(best_board.board_state)
  print(f"Number was {number_on_board_completion}")
  print(f"Best board score is {best_board_score}")
