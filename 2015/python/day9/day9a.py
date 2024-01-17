#!/usr/bin/env python3

from itertools import permutations
from pprint import pprint
import random
import re

input_re = re.compile(r"(\w+)\s+to\s+(\w+)\s+=\s+(\d+)")

paths = {}
nodes = set()

for line in open("input").readlines():
    line = line.strip()
    match = input_re.match(line)
    if not match:
        raise SystemExit(f"{line} failed to match")

    place1 = match.group(1)
    place2 = match.group(2)
    distance = int(match.group(3))

    paths[(place1, place2)] = distance
    paths[(place2, place1)] = distance
    nodes.add(place1)
    nodes.add(place2)

options = dict()

for p in permutations(nodes):
    if p in options or reversed(p) in options:
        continue

    distance = sum(paths[pair] for pair in zip(p, p[1:]))

    options[p] = distance

possibilities = sorted(options.items(), key=lambda path: path[1], reverse=True)
print(possibilities[0])
