#!/usr/bin/env python3

import sys


def calculate_fuel_for_mass(mass):
    return (mass // 3) - 2


def calculate_fuel_requirement(mass):
    total = 0
    while True:
        fuel = calculate_fuel_for_mass(mass)
        if fuel <= 0:
            break
        total += fuel
        mass = fuel
    return total


def main():
    total = 0

    for line in sys.stdin.readlines():
        line = line.strip()
        value = int(line)
        result = calculate_fuel_requirement(value)
        print(result)
        total += result

    print('-----')
    print(total)


if __name__ == '__main__':
    main()
