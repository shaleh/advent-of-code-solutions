#!/usr/bin/env python3

import sys

def calculate_fuel_requirement(value):
    return (value // 3) - 2


total = 0

for line in sys.stdin.readlines():
    line = line.strip()
    value = int(line)
    result = calculate_fuel_requirement(value)
    print(result)
    total += result

print('-----')
print(total)
