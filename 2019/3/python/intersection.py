#!/usr/bin/env python3

compass = {
    'D': lambda p, v: (p[0],     p[1] - v),
    'L': lambda p, v: (p[0] - v, p[1]),
    'R': lambda p, v: (p[0] + v, p[1]),
    'U': lambda p, v: (p[0],     p[1] + v),
}


def walk_path(paths):
    begin_pos = (0, 0)
    end_pos = None

    h_segments = []
    v_segments = []

    for item in paths:
        print(item, begin_pos)
        direction = item[0]
        length = int(item[1:])

        end_pos = compass[direction](begin_pos, length)
        a, b = sorted([begin_pos, end_pos])

        if begin_pos[0] == end_pos[0]:
            v_segments.append(((a, b), 'v'))
        else:
            h_segments.append(((a, b), 'b'))
            h_segments.append(((b, a), 'e'))

        begin_pos = end_pos

    print('------')
    print(end_pos)
    print()

    return h_segments, v_segments


def find_intersections(segments):
    intersections = []

    points = set()

    print(segments)
    for (begin, end), kind in segments:
        if kind == 'b':
            points.add(begin)
        elif kind == 'e':
            try:
                points.remove(end)
            except KeyError:
                pass
        elif kind == 'v':
            print(points)
            options = [(begin[0], y) for _, y in points if y >= begin[1] and y <= end[1]]
            for i in options:
                length = abs(i[0]) + abs(i[1])
                if length != 0:
                    intersections.append((i, length))

    return sorted(intersections, key=lambda x: x[1])


def main():
    import sys
    with open(sys.argv[1]) as fp:
        path1 = fp.readline().strip().split(',')
        path2 = fp.readline().strip().split(',')

    h_segments1, v_segments1 = walk_path(path1)
    h_segments2, v_segments2 = walk_path(path2)

    segments1 = sorted(h_segments1 + v_segments2, key=lambda x: x[0][0][0])
    segments2 = sorted(h_segments2 + v_segments1, key=lambda x: x[0][0][0])

    intersections1 = find_intersections(segments1)
    intersections2 = find_intersections(segments2)

    print(intersections1)
    print(intersections2)
    if intersections1[0][1] <= intersections2[0][1]:
        shortest = intersections1[0]
    else:
        shortest = intersections2[0]

    closest, distance = shortest
    print(closest, distance)


if __name__ == '__main__':
    main()
