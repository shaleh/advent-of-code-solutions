#!/usr/bin/env python3

import math
import sys

def main():

    def init_asteroids():
        with open(sys.argv[1]) as f:
            for y, line in enumerate(f.readlines()):
                for x, a in enumerate(line):
                    if a == '#':
                        yield (x, y)

    asteroids = list(init_asteroids())

    def angle(start, end):
        result = math.degrees(math.atan2(end[0] - start[0], start[1] - end[1]))
        if result < 0:
            result += 360.0
        return result

    # part 1
    result = None
    m = 0

    for start in asteroids:
        cnt = len({angle(start, end) for end in asteroids if start != end})
        if cnt > m:
            m = cnt
            result = start

    print('x {} y {}'.format(*result))
    print('visible {}'.format(m))

    # part 2
    asteroids.remove(result)
    angles = sorted(
        ((angle(result, end), end) for end in asteroids),
        key=lambda x: (x[0], abs(result[0] - x[1][0]) + abs(result[1] - x[1][1]))
    )
    for k in angles:
        print(k)
    idx = 0
    last = angles.pop(idx)
    last_angle = last[0]
    cnt = 1
    print('---')
    while cnt < 200 and angles:
        print(idx, last, last_angle, cnt)
        if idx >= len(angles):
            idx = 0
            last_angle = None
        if last_angle == angles[idx][0]:
            idx += 1
            continue
        last = angles.pop(idx)
        last_angle = last[0]
        cnt += 1
    print('vaporized {}: {} {}'.format(cnt, last[1], last[1][0] * 100 + last[1][1]))


if __name__ == '__main__':
    main()
