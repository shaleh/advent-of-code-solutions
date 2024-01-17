#!/usr/bin/env python3

"""
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
"""

input = [line.strip() for line in open("input").readlines()]


def race(input, contest_duration):
    contestants = {}

    for line in input:
        parts = line.split()
        deer = parts[0]
        speed = int(parts[3])
        duration = int(parts[6])
        rest = int(parts[-2])

        sprints, remaining = divmod(contest_duration, (duration + rest))
        remaining = min(duration, remaining)
        distance = (speed * duration * sprints) + (speed * remaining)

        contestants[deer] = distance

    return sorted(contestants.items(), key=lambda d: d[1], reverse=True)[0]


print(
    race(
        [
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        ],
        1_000,
    )
)

print(race(input, 2503))
