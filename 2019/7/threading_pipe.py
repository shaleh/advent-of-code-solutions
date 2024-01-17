import random
from multiprocessing import Pipe
from select import select
from threading import Thread


def main():
    c1_r, c1_w = Pipe(duplex=False)
    c2_r, c2_w = Pipe(duplex=False)
    quit_r, quit_w = Pipe(duplex=False)

    def func1():
        for i in range(10):
            c1_w.send(i)
        quit_w.send(0)

    Thread(target=func1).start()

    def func2():
        for i in range(2):
            c2_w.send(i)

    Thread(target=func2).start()

    while True:
        ready, _, _ = select([c1_r, c2_r, quit_r], [], [])
        for which in ready:
            if which == c1_r:
                value = c1_r.recv()
                print(f'Received {value} from c1')
            elif which == c2_r:
                value = c2_r.recv()
                print(f'Received {value} from c2')
            elif which == quit_r:
                value = quit_r.recv()
                print(f'Received {value} from quit')
                return


if __name__ == '__main__':
    main()
