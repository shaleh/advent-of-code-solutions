#!/usr/bin/env python3
import copy
import itertools
import sys
from io import StringIO

from intcode import run_evaluate, parse

with open(sys.argv[1]) as fp:
    input = parse(fp.read())

print(input)

outputs = []

for i, pattern in enumerate(itertools.permutations(range(5))):
    value = 0
    print(pattern)

    for setting in pattern:
        # print("Using:", setting, value)
        pipe_stdin = StringIO('\n'.join([str(setting), str(value)]) + '\n')
        pipe_stdout = StringIO()

        run_evaluate(copy.copy(input), {'instruction_pointer': 0},
                     stdin=pipe_stdin,
                     stdout=pipe_stdout)

        value = pipe_stdout.getvalue().split()[-1]

    outputs.append((pattern, int(value)))
    print("Final:", i, value)

outputs.sort(key=lambda x: x[1])
print(outputs[0], outputs[-1])
