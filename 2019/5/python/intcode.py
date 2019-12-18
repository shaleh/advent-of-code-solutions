#!/usr/bin/env python3

import operator
from functools import partial


def parse(input):
    # first, collapse newlines and whitespace.
    input = ''.join(input.split())
    # Now parse the CSV.
    return [int(x.strip()) for x in input.strip().split(',')]


def op_terminate(state, pointer, op, memory, _modes):
    new_state = {'should_eval': False}
    return new_state


def mode_for_argument(modes, position):
    try:
        return modes[position]
    except IndexError:
        return 0


def op_binop(op_action, state, pointer, op, memory, modes):
    if not state.get('should_eval', True):
        return {}

    value1 = memory[pointer + 1]
    if mode_for_argument(modes, 0) == 0:
        value1 = memory[value1]
    value2 = memory[pointer + 2]
    if mode_for_argument(modes, 1) == 0:
        value2 = memory[value2]

    result = op_action(value1, value2)
    new_state = {
        'result_pointer': memory[pointer + 3],
        'result': result,
    }
    return new_state


def op_input(state, pointer, op, memory, _modes):
    if state.get('should_eval', True):
        result = int(input("> "))
        new_state = {
            'result_pointer': memory[pointer + 1],
            'result': result,
        }
        return new_state

    return {}


def op_output(state, pointer, op, memory, modes):
    if not state.get('should_eval', True):
        return {}

    value = memory[pointer + 1]
    if mode_for_argument(modes, 0) == 0:
        value = memory[value]
    print(value)
    return {'result_pointer': None}


ops = {
    99: (1, False, op_terminate),
    1: (4, True, partial(op_binop, operator.add)),
    2: (4, True, partial(op_binop, operator.mul)),
    3: (2, False, op_input),
    4: (2, True, op_output),
}


def evaluate(state, pointer, op, memory):
    if op > 99:
        base = str(op)
        op = int(base[-2:])
        modes = list(reversed([c for c in base[:-2]]))
    else:
        modes = []

    try:
        pointer_increment, supports_modes, action = ops[op]
        if modes and not supports_modes:
            raise NotImplementedError(f"attempt to use modes for {op}")
    except KeyError:
        raise NotImplementedError(op)

    new_state = action(state, pointer, op, memory, modes)
    new_state['instruction_pointer'] = pointer + pointer_increment
    return new_state


def run_evaluate(input, state):
    while True:
        pointer = state['instruction_pointer']
        if pointer >= len(input):
            return input

        new_state = evaluate(state, pointer, input[pointer], input)
        if new_state.get('should_eval', True) is False:
            break

        result_pointer = new_state.get('result_pointer')
        if result_pointer is not None:
            input[result_pointer] = new_state.get('result')

        state.update(**new_state)

    return input


def pretty_print_state(code):
    print(code)


def run(input):
    return run_evaluate(parse(input), {'instruction_pointer': 0})


def main(inputfile):
    test()
    input = open(inputfile).read()
    code = parse(input)
    pretty_print_state(code)
    print('---')
    pretty_print_state(run_evaluate(code, {'instruction_pointer': 0}))


def test():
    assert run('1,0,0,0,99') == [2,0,0,0,99]
    result = run('''
1,9,10,3,
2,3,11,0,
99,
30,40,50
''')
    print(result)
    assert result == [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    assert run('2,3,0,3,99') == [2,3,0,6,99]
    assert run('2,4,4,5,99,0') == [2,4,4,5,99,9801]
    assert run('1,1,1,4,99,5,6,0,99') == [30,1,1,4,2,5,6,0,99]
    print('test successful')


if __name__ == '__main__':
    import sys
    main(sys.argv[1])
