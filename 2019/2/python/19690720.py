#!/usr/bin/env python3

import copy

from intcode import parse, run_evaluate


def find_pair(code, value):
    noun = 0
    verb = 0
    while True:
        memory = copy.deepcopy(code)
        memory[1] = noun
        memory[2] = verb
        result = run_evaluate(memory, {'instruction_pointer': 0})
        if result[0] == value:
            return (result[1], result[2])

        verb += 1
        if verb > 99:
            verb = 0
            noun += 1
            if noun > 99:
                raise RuntimeError("no value found")


def main(inputfile):
    input = inputfile

    code = parse(input)
    noun, verb = find_pair(code, 19690720)
    print(noun, verb)
    print(100 * noun + verb)


if __name__ == '__main__':
    import sys
    main(sys.argv[1])
