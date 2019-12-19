#!/usr/bin/env python3

import copy
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


class Memory:
    def __init__(self, program):
        self.program = copy.copy(program)
        self.auxilliary = {}
        self.relative_base = 0

    def access(self, position, mode):
        data_position = self._compute_position(position, mode)
        return self._direct_access(data_position)

    def _compute_position(self, value, mode):
        if mode == 0:
            data_position = self._direct_access(value)
        elif mode == 1:
            data_position = value
        elif mode == 2:
            data_position = self.relative_base + self._direct_access(value)
        else:
            raise NotImplementedError(f"argument mode {mode}")

        return data_position

    def _direct_access(self, position):
        if position > len(self.program):
            return self.auxilliary.get(position, 0)
        return self.program[position]

    def store(self, position, value):
        if position > len(self.program):
            self.auxilliary[position] = value
        else:
            self.program[position] = value

    def size(self):
        return len(self.program)

    def dump(self):
        return self.program

    def adjust_base(self, value):
        self.relative_base += value
        if self.relative_base < 0:
            raise RuntimeError("relative_base cannot be less than zero")


def op_binop(op_action, state, pointer, op, memory, modes):
    if not state.get('should_eval', True):
        return {}

    value1 = memory.access(pointer + 1, mode_for_argument(modes, 0))
    value2 = memory.access(pointer + 2, mode_for_argument(modes, 1))

    result = op_action(value1, value2)
    new_state = {
        'result_pointer': memory._compute_position(pointer + 3, mode_for_argument(modes, 2)),
        'result': result,
    }
    return new_state


def op_input(state, pointer, op, memory, modes):
    if not state.get('should_eval', True):
        return {}

    result = int(input("> "))
    result_pointer = memory._compute_position(pointer + 1,
                                              mode_for_argument(modes, 0))

    new_state = {
        'result_pointer': result_pointer,
        'result': result,
    }
    return new_state


def op_output(state, pointer, op, memory, modes):
    if not state.get('should_eval', True):
        return {}

    value = memory.access(pointer + 1, mode_for_argument(modes, 0))
    print(value)
    return {'result_pointer': None}


def op_jump_if(op_action, state, pointer, op, memory, modes):
    value1 = memory.access(pointer + 1, mode_for_argument(modes, 0))
    value2 = memory.access(pointer + 2, mode_for_argument(modes, 1))
    if op_action(value1):
        return {'jump': value2}
    return {}


def op_boolean(op_action, state, pointer, op, memory, modes):
    if not state.get('should_eval', True):
        return {}

    value1 = memory.access(pointer + 1, mode_for_argument(modes, 0))
    value2 = memory.access(pointer + 2, mode_for_argument(modes, 1))
    result = op_action(value1, value2)
    return {
        'result_pointer': memory._compute_position(pointer + 3,
                                                   mode_for_argument(modes, 2)),
        'result': int(result),
    }


def op_adjust_base(state, pointer, op, memory, modes):
    value = memory.access(pointer + 1, mode_for_argument(modes, 0))
    return {'adj_relative_base': value}


ops = {
    99: (1, op_terminate),
    1: (4, partial(op_binop, operator.add)),
    2: (4, partial(op_binop, operator.mul)),
    3: (2, op_input),
    4: (2, op_output),
    5: (3, partial(op_jump_if, partial(operator.ne, 0))),
    6: (3, partial(op_jump_if, partial(operator.eq, 0))),
    7: (4, partial(op_boolean, operator.lt)),
    8: (4, partial(op_boolean, operator.eq)),
    9: (2, op_adjust_base),
}


def emit_failure(state, memory, pointer, op, message):
    raise RuntimeError({
        'state': state,
        'memory': memory.dump(),
        'pointer': pointer,
        'op': op,
        'message': message,
    })


def evaluate(state, pointer, op, memory):
    if op > 99:
        base = str(op)
        op = int(base[-2:])
        modes = list(reversed([int(c) for c in base[:-2]]))
    else:
        modes = []

    try:
        pointer_increment, action = ops[op]
    except KeyError:
        raise NotImplementedError(op)

    new_state = action(state, pointer, op, memory, modes)
    new_state['instruction_pointer'] = pointer + pointer_increment
    return new_state


def run_evaluate(input, state):
    memory = Memory(input)

    while True:
        try:
            pointer = state['jump']
            del state['jump']
        except KeyError:
            pointer = state['instruction_pointer']

        if pointer >= memory.size():
            return memory.dump()

        op = memory._direct_access(pointer)
        new_state = evaluate(state, pointer, op, memory)
        if new_state.get('should_eval', True) is False:
            break

        # print(op, new_state)

        result_pointer = new_state.get('result_pointer')
        if result_pointer is not None:
            memory.store(result_pointer, new_state.get('result'))

        try:
            adjustment = new_state['adj_relative_base']
            memory.adjust_base(adjustment)
        except KeyError:
            pass
        state.update(**new_state)

    return memory.dump()


def pretty_print_state(code):
    print(code)


def run(input):
    return run_evaluate(parse(input), {'instruction_pointer': 0})


def main(args):
    inputfile = args[-1]
    input = open(inputfile).read()
    code = parse(input)
    pretty_print_state(code)
    print('---')
    pretty_print_state(run_evaluate(code, {'instruction_pointer': 0}))


if __name__ == '__main__':
    import sys
    import unittest

    class TestSmokeTests(unittest.TestCase):
        def test_1(self):
            self.assertEqual(run('1,0,0,0,99'), [2,0,0,0,99])

        def test_2(self):
            self.assertEqual(run('2,3,0,3,99'), [2,3,0,6,99])

        def test_3(self):
            self.assertEqual(run('2,4,4,5,99,0'), [2,4,4,5,99,9801])

        def test_4(self):
            self.assertEqual(run('1,1,1,4,99,5,6,0,99'), [30,1,1,4,2,5,6,0,99])

        def test_5(self):
            result = run('''
1,9,10,3,
2,3,11,0,
99,
30,40,50
''')

            self.assertEqual(result, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50])

        def test_6(self):
            result = run('1101,1,1,5,99,0')
            self.assertEqual(result, [1101, 1, 1, 5, 99, 2])

        def test_7(self):
            result = run('1101,1,1,1,109,2,2201,1,1,11,99,0')
            # 1 + 1 => 2, change offset to point to destination argument of first op.
            # 1 + 1 => 2 again, but a different set of 1s.
            self.assertEqual(result, [1101, 2, 1, 1, 109, 2, 2201, 1, 1, 11, 99, 2])

        # def test_8(self):
        #     result = run('109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99')
        #     import pdb; pdb.set_trace()

    args = ['intcode', sys.argv.pop()]

    # Run smoke tests first.
    result = unittest.main(exit=False)
    if not result.result.wasSuccessful():
        raise SystemExit(1)

    main(args)
