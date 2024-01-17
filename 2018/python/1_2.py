from __future__ import print_function

import sys

result = 0

seen = {}

values = [int(line.strip()) for line in open(sys.argv[1]).readlines()]

count = 0
while True:
    count += 1
    for value in values:
        result += value
        print(f"{count}: {result}")
        if result in seen:
            raise SystemExit("Repeat")

        seen[result] = True
