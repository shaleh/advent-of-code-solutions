#!/usr/bin/env python3
import copy
import itertools
import sys
import threading
from multiprocessing import Pipe
from threading import Thread

from intcode import run_evaluate, parse


def main():
    c1_r, c1_w = Pipe(duplex=False)
    c2_r, c2_w = Pipe(duplex=False)
    quit_r, quit_w = Pipe(duplex=False)

    def func1():
        for i in range(10):
            c1_w.send(i)
        quit_w.send(0)

    Thread(target=func1).start()


def run_one_amplifier(program, index, stdin, stdout):
    data = threading.local()
    data.program = copy.copy(program)
    run_evaluate(data.program, {'instruction_pointer': 0},
                 stdin=stdin, stdout=stdout)

    return

# def run_one_amplifier(program, index, stdin, stdout):
#      data = threading.local()
#      data.program = copy.copy(program)

#      setting = stdin.recv()
#      print(f"{index}: settings {setting}")
#      value = stdin.recv()
#      print(f"{index}: value {value}")
#      stdout.send(value + 1)

#      return


def run_iteration(pattern, program):
    value = 0
    # print(pattern)

    pipe1_read, pipe1_write = Pipe(duplex=False)
    pipe2_read, pipe2_write = Pipe(duplex=False)
    pipe3_read, pipe3_write = Pipe(duplex=False)
    pipe4_read, pipe4_write = Pipe(duplex=False)
    pipe5_read, pipe5_write = Pipe(duplex=False)
    t1 = Thread(target=run_one_amplifier, args=(program, 1, pipe1_read, pipe2_write))
    t2 = Thread(target=run_one_amplifier, args=(program, 2, pipe2_read, pipe3_write))
    t3 = Thread(target=run_one_amplifier, args=(program, 3, pipe3_read, pipe4_write))
    t4 = Thread(target=run_one_amplifier, args=(program, 4, pipe4_read, pipe5_write))
    t5 = Thread(target=run_one_amplifier, args=(program, 5, pipe5_read, pipe1_write))

    pipe1_write.send(pattern[0])
    pipe1_write.send(value)
    pipe2_write.send(pattern[1])
    pipe3_write.send(pattern[2])
    pipe4_write.send(pattern[3])
    pipe5_write.send(pattern[4])

    t1.start()
    t2.start()
    t3.start()
    t4.start()
    t5.start()

    t5.join()
    t4.join()
    t3.join()
    t2.join()
    t1.join()

    value = pipe1_read.recv()
    print(f"Return: {value}")
    return int(value)


def main(inputfile):
    with open(inputfile) as fp:
        input = parse(fp.read())

    print(input)

    outputs = []

    for i, pattern in enumerate(itertools.permutations(range(5, 10))):
        value = run_iteration(pattern, input)
        print("Final:", i, value)
        outputs.append((pattern, value))

    outputs.sort(key=lambda x: x[1])
    print(outputs[0], outputs[-1])
    import pdb; pdb.set_trace()


if __name__ == '__main__':
    main(sys.argv[1])
