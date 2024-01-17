#!/usr/bin/env python3

import re

num_re = re.compile(r"(\-?\d+)")
data = open("input").read()
total = sum([int(d) for d in num_re.findall(data)])
print(total)
