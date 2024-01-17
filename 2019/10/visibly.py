#!/usr/bin/env python3

import math


def find_asteroids(data):
    asteroids = []

    for y, row in enumerate(data):
        for x, column in enumerate(data[y]):
            if data[y][x] == '#':
                asteroids.append((x, y))

    return asteroids


def angle_of(p1, p2):
    dX = p2[0] - p1[0]
    dY = p1[1] - p2[1]  # Y axis is flipped versus traditional Cartesian
    radians = math.atan2(dX, dY)
    result = math.degrees(radians)
    if result < 0:
        result += 360.0

    return result


def compute_angles_to_other_asteroids(asteroids, position):
    for asteroid in asteroids:
        if asteroid == position:
            continue  # skip self

        angle = angle_of(position, asteroid)
        yield (angle, (abs(position[0] - asteroid[0]), abs(position[1] - asteroid[1])), asteroid)


def next_victim(queue, victims):
    while queue:
        angle = queue.pop(0)
        remaining = victims[angle]
        victim = remaining.pop(0)
        yield (angle, victim)
        if remaining:
            queue.append(angle)


def main(inputfile):
    with open(inputfile) as fp:
        data = [[c for c in l.strip()] for l in fp.readlines()]

    for row in data:
        print(row)

    asteroids = find_asteroids(data)
    print(asteroids, len(asteroids))

    visibilities = []
    for idx, asteroid in enumerate(asteroids):
        thing = {}
        for angle, distances, other in compute_angles_to_other_asteroids(asteroids, asteroid):
            thing.setdefault(angle, []).append((other, distances))
        visibilities.append((idx, thing))

    visibilities.sort(key=lambda x: len(x[1]))
    idx, info = visibilities[-1]
    print(asteroids[idx], len(info))

    info = {angle: list(sorted(visible, key=lambda x: x[1])) for angle, visible in info.items()}
    angles = list(sorted(info.keys()))
    count = 1
    while angles:
        angle = angles.pop(0)
        asteroid, _ = info[angle].pop(0)
        print(count, angle, asteroid)
        if info[angle]:
            angles.append(angle)
        count += 1

if __name__ == '__main__':
    import sys

    # print(angle_of((2, 2), (1, 1)))
    # raise SystemExit()
    main(sys.argv[1])
