#!/usr/bin/env python3

"""
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
"""


def parse(input):
    contestants = {}

    for line in input:
        parts = line.split()
        deer = parts[0]
        speed = int(parts[3])
        duration = int(parts[6])
        rest = int(parts[-2])

        contestants[deer] = (speed, duration, rest)

    return contestants


def race(contestants, contest_duration):
    results = {}

    positions = {
        deer: (0, duration, 0) for deer, (_, duration, _) in contestants.items()
    }

    for i in range(contest_duration):
        for deer, (speed, duration, rest) in contestants.items():
            current_position, move_time_remaining, rest_time_remaining = positions[deer]
            if move_time_remaining:
                current_position += speed
                move_time_remaining -= 1
                if not move_time_remaining:
                    rest_time_remaining = rest
            else:
                rest_time_remaining -= 1
                if not rest_time_remaining:
                    move_time_remaining = duration

            positions[deer] = (
                current_position,
                move_time_remaining,
                rest_time_remaining,
            )

        ordered = sorted(positions.items(), key=lambda p: p[1][0], reverse=True)
        distance = ordered[0][1][0]
        for deer, (current_distance, _, _) in ordered:
            if current_distance != distance:
                break
            results[deer] = results.get(deer, 0) + 1

    return results


data = parse(
    [
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
    ],
)
print(race(data, 1_000))

input = [line.strip() for line in open("input").readlines()]
data = parse(input)
results = race(data, 2503)
print(sorted(results.items(), key=lambda r: r[1], reverse=True))
print(sum(results.values()))
