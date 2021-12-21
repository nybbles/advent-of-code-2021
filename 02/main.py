#! python

from enum import Enum
from functools import reduce

Instruction = Enum("Instruction", "forward down up")
Distance = int
Command = tuple[Instruction, Distance]

HorizontalPosition = int
Depth = int
State = tuple[HorizontalPosition, Depth]


def parse_command(line) -> Command:
  raw_instruction, raw_distance = line.split()
  instruction = Instruction[raw_instruction]
  distance = int(raw_distance)
  return (instruction, distance)


def state_after_command(state: State, command: Command):
  horizonal_position, depth = state
  match command:
    case (Instruction.forward, distance):
      return (horizonal_position + distance, depth)
    case (Instruction.down, distance):
      return (horizonal_position, depth + distance)
    case (Instruction.up, distance):
      return (horizonal_position, depth - distance)
    

input = open('02/input.txt', 'r').readlines()
commands = [parse_command(x) for x in input]
state = (0, 0)

end_state = reduce(state_after_command, commands, state)
print(end_state)



