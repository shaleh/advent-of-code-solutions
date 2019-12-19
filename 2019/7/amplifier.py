#!/usr/bin/env python3
import copy
import itertools
import sys
from io import StringIO

from intcode import run_evaluate, parse


def run_one_amplifier(input, setting, initial_value):
    pipe_stdin = StringIO('\n'.join([str(setting), str(initial_value)]) + '\n')
    pipe_stdout = StringIO()

    run_evaluate(copy.copy(input), {'instruction_pointer': 0},
                 stdin=pipe_stdin,
                 stdout=pipe_stdout)

    return pipe_stdout.getvalue().split()[-1]


def run_iteration(pattern, input):
    value = 0
    print(pattern)

    for setting in pattern:
        value = run_one_amplifier(input, setting, value)

    return int(value)


def main(inputfile):
    with open(inputfile) as fp:
        input = parse(fp.read())

    print(input)

    outputs = []

    for i, pattern in enumerate(itertools.permutations(range(5))):
        value = run_iteration(pattern, input)
        print("Final:", i, value)
        outputs.append((pattern, value))

    outputs.sort(key=lambda x: x[1])
    print(outputs[0], outputs[-1])


if __name__ == '__main__':
    main(sys.argv[1])
