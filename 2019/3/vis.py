#!/usr/bin/env python

import sys
import turtle

def rotate(previous, direction):
    if direction == 'R':
        if previous == 'U':
            return 90
        elif previous == 'D':
            return -90
        elif previous == 'R':
            return 0
    elif direction == 'L':
        if previous == 'U':
            return -90
        elif previous == 'D':
            return 90
    elif direction == 'D':
        if previous == 'L':
            return -90
        elif previous == 'R':
            return 90
    elif direction == 'U':
        if previous == 'L':
            return 90
        elif previous == 'R':
            return -90

    raise RuntimeError((previous, direction))

cursor = turtle.Turtle()

previous = 'R'

for line in open(sys.argv[1]).readlines():
    paths = line.strip().split(',')
    for item in paths:
        direction = item[0]
        length = int(item[1:])
        cursor.setheading(rotate(previous, direction))
        cursor.forward(length)

cursor.done()
