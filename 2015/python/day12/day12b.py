#!/usr/bin/env python3

import json


def get_numbers(json_data):
    if isinstance(json_data, int):
        return json_data
    elif isinstance(json_data, str):
        return 0
    elif isinstance(json_data, dict):
        total = 0

        values = list(json_data.values())
        if "red" in values:
            return 0
        for v in values:
            total += get_numbers(v)

        return total
    elif isinstance(json_data, list):
        return sum(get_numbers(d) for d in json_data)
    else:
        raise NotImplemented(f"what is this? {type(json_data)}")


data = json.load(open("input"))
print(get_numbers(data))
