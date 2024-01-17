#!/usr/bin/env python3

import itertools
from collections import deque


def parse(input):
    data = {}
    for obj1, obj2 in input:
        data.setdefault(obj1, []).append(obj2)
        data.setdefault(obj2, []).append(obj1)

    return data


def walk(data, start, terminal, action, state):
    current = start

    while True:
        state = action(state, current)

        if current == terminal:
            break

        next = data[current][0]
        current = next

    return state


def print_node(state, node):
    print(node)
    return state


def counter(state, node):
    return state + 1


def build_path(state, node):
    return state + [node]


def find_path(graph, start, end, path=[]):
    path = path + [start]
    if start == end:
        return path
    if start not in graph:
        return None
    node = graph[start]
    if node not in path:
        print('Path:', path)
        newpath = find_path(graph, node, end, path)
        print('Newpath:', newpath)
        if newpath: return newpath
    return None

def find_shortest_path(graph, start, end, path=[]):
    path = path + [start]
    if start == end:
        return path
    if start not in graph:
        return None
    shortest = None
    for node in graph[start]:
        if node not in path:
            newpath = find_shortest_path(graph, node, end, path)
            if newpath:
                if not shortest or len(newpath) < len(shortest):
                    shortest = newpath
    return shortest


def flatten(values):
    current = values
    tails = []

    while True:
        front, *end = current
        if not end:
            tails.append(front)
            break
        tails.append(end[0])
        current = front

    return list(reversed(tails))


def find_shortest_path2(graph, start, end):
    dist = {start: [start]}
    q = deque([start])
    while len(q):
        at = q.popleft()
        for next in graph[at]:
            if next not in dist:
                dist[next] = [dist[at], next]
                q.append(next)
    result = flatten(dist.get(end))
    return result


def main(input):
    data = parse(input)
    san2 = find_shortest_path2(data, 'SAN', 'YOU')
    you2 = find_shortest_path2(data, 'YOU', 'SAN')
    print(you2)
    print(len(san2) - 2 - 1)
    print(len(you2) - 2 - 1)


if __name__ == '__main__':
    import sys
    with open(sys.argv[1]) as fp:
        input = [line.strip().split(')') for line in fp.readlines()]
        main(input)
