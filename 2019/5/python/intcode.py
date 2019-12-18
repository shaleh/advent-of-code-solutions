#!/usr/bin/env python3


def parse(input):
    # first, collapse newlines and whitespace.
    input = ''.join(input.split())
    # Now parse the CSV.
    return [int(x.strip()) for x in input.strip().split(',')]


def evaluate_bin_op(state, op, param1, param2):
    if op == 1:
        return param1 + param2
    elif op == 2:
        return param1 * param2
    else:
        raise NotImplementedError(op)


def evaluate(state, pointer, op, memory):
    new_state = {}

    if op == 99:
        if state.get('print'):
            print('99,')
        else:
            new_state['should_eval'] = False
        new_state['instruction_pointer'] = pointer + 1
    elif op in (1, 2):
        if state.get('print'):
            print(','.join([str(x) for x in memory[pointer:pointer + 3 + 1]]) + ',')
        elif state.get('should_eval', True):
            address1, address2 = memory[pointer + 1:pointer + 2 + 1]
            result = evaluate_bin_op(state, op, memory[address1], memory[address2])
            new_state['result_pointer'] = memory[pointer + 3]
            new_state['result'] = result
        new_state['instruction_pointer'] = pointer + 4
    elif op == 3:
        if state.get('print'):
            print(','.join([str(x) for x in memory[pointer:pointer + 1 + 1]]) + ',')
        elif state.get('should_eval', True):
            result = int(input("> "))
            new_state['result_pointer'] = memory[pointer + 1]
            new_state['result'] = result
        new_state['instruction_pointer'] = pointer + 2
    elif op == 4:
        if state.get('print'):
            print(','.join([str(x) for x in memory[pointer:pointer + 1 + 1]]) + ',')
        elif state.get('should_eval', True):
            print(memory[memory[pointer + 1]])
            new_state['result_pointer'] = None
        new_state['instruction_pointer'] = pointer + 2
    else:
        if state.get('print'):
            print(','.join([str(x) for x in memory[pointer:pointer+3+1]]) + ',')
        elif state.get('should_eval', True):
            raise NotImplementedError(op)
        new_state['instruction_pointer'] = pointer + 4

    return new_state


def run_evaluate(input, state):
    while True:
        pointer = state['instruction_pointer']
        if pointer >= len(input):
            return input
        new_state = evaluate(state, pointer, input[pointer], input)
        if new_state.get('should_eval', True):
            result_pointer = new_state.get('result_pointer')
            if result_pointer is not None:
                input[result_pointer] = new_state.get('result')

        state.update(**new_state)

    return input


def pretty_print_state(code):
    run_evaluate(code, {'instruction_pointer': 0, 'print': True})


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
