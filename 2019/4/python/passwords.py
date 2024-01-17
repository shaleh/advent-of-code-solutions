#!/usr/bin/env python3

from collections import Counter
from itertools import repeat


def next_potential(pairs):
    accepted = []

    for a, b in pairs:
        if b < a:
            accepted.append((a, a))
            break
        accepted.append((a, b))

    if accepted:
        accepted.extend(repeat((accepted[-1][1], accepted[-1][1]),
                               len(pairs) - len(accepted)))
    return accepted


def valid(pairs):
    ascending = all(a <= b for (a, b) in pairs)
    if ascending:
        multiples = Counter([(a, b) for a, b in pairs if a == b])
        return 1 in multiples.values()

    return False


def reconstitute(pairs):
    values = [str(a) for a, _ in pairs]
    values.extend([str(pairs[-1][1])])
    return int(''.join(values))


def increment(pairs):
    number = reconstitute(pairs)
    number += 1
    return make_pairs(number)


def generate_pairs(begin, end):
    current = begin

    while current <= end:
        if valid(current):
            yield current
            current = increment(current)
        else:
            next = next_potential(current)
            if next == current:
                next = increment(next)
            current = next


def make_pairs(number: int) -> [(int, int)]:
    value = [int(x) for x in str(number)]
    return list(zip(value, value[1:]))


def foo(begin, end):
    begin_pairs = make_pairs(begin)
    print(begin_pairs)
    first = first_in_pattern(begin_pairs)
    if not first:
        first = list(repeat(begin_pairs[0][0] + 1, len(begin_pairs) + 1))

    end_pairs = make_pairs(end)
    last = first_in_pattern(end_pairs)
    if not last:
        last = [end_pairs[0][0] - 1] + list(repeat(9, len(begin_pairs)))

    print(first)
    print(last)


def parse(input):
    begin, end = [int(x.strip()) for x in input.split('-')]
    return (make_pairs(begin), make_pairs(end))


def main(inputfile):
    input = open(inputfile).read()
    begin, end = parse(input)


if __name__ == '__main__':
    import sys
    main(sys.argv[1])
