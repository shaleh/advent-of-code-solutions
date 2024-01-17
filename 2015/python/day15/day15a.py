#!/usr/bin/env python3

"""
Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5
"""

import re
import sys

input_re = re.compile(r'(\w+):\s+capacity\s+(-?\d+),\s+durability\s+(-?\d+),\s+flavor\s+(-?\d+),\s+texture\s+(-?\d+),\s+calories\s+(-?\d+)')

ingredients = {}
for line in sys.stdin.readlines():
    line = line.strip()
    match = input_re.match(line)
    name, *values = match.groups()
    ingredients[name] = tuple(int(x) for x in values)

print(ingredients)
#for key in ingredients:
    
