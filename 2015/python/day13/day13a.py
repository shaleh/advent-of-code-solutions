#!/usr/bin/env python

"""
XYZ would lose/gain DD happiness units by sittings next to ABC.
"""
from itertools import permutations
from pprint import pprint

edges = {}
nodes = set()
for line in open("input").readlines():
    line = line.strip()
    parts = line.split(" ")
    person1 = parts[0]
    person2 = parts[-1][:-1]
    nodes.add(person1)
    nodes.add(person2)
    happiness = int(parts[3]) * (parts[2] == "gain" and 1 or -1)
    edges[(person1, person2)] = happiness

options = {}
for p in permutations(nodes):
    if p in options or reversed(p) in options:
        continue

    pairs = zip(p, p[1:])
    pairs.append((pairs[-1][1], pairs[0][0]))

    options[tuple(x[0] for x in pairs)] = sum(
        edges[p] + edges[tuple(reversed(p))] for p in pairs
    )

optimal = sorted(options.items(), key=lambda p: p[1], reverse=True)[0]
print(optimal)
