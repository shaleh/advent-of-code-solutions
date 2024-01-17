#!/usr/bin/env python

from itertools import groupby

input = "1321131112"


def look_and_say(input):
    return "".join(
        "{}{}".format((len(list(group))), num) for num, group in groupby(input)
    )


for idx, _ in enumerate(range(50)):
    input = look_and_say(input)

print(idx, len(input))
