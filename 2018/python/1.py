from __future__ import print_function

import sys

result = 0

values = [int(line.strip()) for line in open(sys.argv[1]).readlines()]

for value in values:
    result += value
    print(result)
