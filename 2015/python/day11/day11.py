#!/usr/bin/env python3

from itertools import accumulate
import operator
import sys


def get_index_of(items, values):
    for v in values:
        try:
            return items.index(v)
        except ValueError:
            pass

    return None


def increment(input):
    chars = [ord(c) for c in input]

    idx = get_index_of(chars, (105, 108, 111))
    if idx is not None:
        chars[idx] += 1
        chars[idx + 1 :] = [97] * (len(chars) - (idx + 1))
        return "".join(chr(c) for c in chars)

    while True:
        for i in range(1, len(chars)):
            chars[-i] += 1
            if chars[-i] in (105, 108, 111):
                chars[-i] += 1

            if chars[-i] < 123:
                break

            chars[-i] = 97

        if not (105 in chars or 108 in chars or 111 in chars):
            break

    return "".join(chr(c) for c in chars)


def is_valid(input):
    chars = [ord(c) for c in input]

    triples = 0
    pairs = 0

    for idx in range(2, len(chars)):
        if chars[idx - 1] - chars[idx - 2] == 1 and chars[idx] - chars[idx - 1] == 1:
            triples += 1
        if chars[idx] == chars[idx - 1] and chars[idx] != chars[idx - 2]:
            pairs += 1

    return triples > 0 and pairs >= 2


def generate_password(input):
    new_password = input

    while True:
        new_password = increment(new_password)

        if is_valid(new_password):
            return new_password


assert generate_password("abcdefgh") == "abcdffaa"
assert generate_password("ghijklmn") == "ghjaabcc"
assert generate_password("vzbxkghb") == "vzbxxyzz"
assert generate_password("vzbxxyzz") == "vzcaabcc"
