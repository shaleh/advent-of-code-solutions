#!/usr/bin/env python3


class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def manhattan_distance(self, other):
        return (abs(self.x) - abs(other.x)) + (abs(self.y) - abs(other.y))

    def __repr__(self):
        return f"({self.x}, {self.y})"

    def __str__(self):
        return repr(self)

    def __add__(self, delta):
        return self.__class__(self.x + delta[0], self.y + delta[1])

    def __lt__(self, other):
        if self.x < other.x:
            return self.y <= other.y
        elif self.x == other.x:
            return self.y < other.y
        else:
            return False


class Segment:
    def __init__(self, begin, end):
        self.begin = begin
        self.end = end

    def __str__(self):
        return f"begin: {self.begin}, end: {self.end}"


compass = {
    'D': lambda p, v: p + ( 0, -v),
    'L': lambda p, v: p + (-v,  0),
    'R': lambda p, v: p + ( v,  0),
    'U': lambda p, v: p + ( 0,  v),
}


def walk_path(paths):
    begin_pos = Point(0, 0)
    end_pos = None

    h_segments = []
    v_segments = []

    for item in paths:
        # print(item, begin_pos)
        direction = item[0]
        length = int(item[1:])

        end_pos = compass[direction](begin_pos, length)
        a, b = sorted([begin_pos, end_pos])

        if begin_pos.x == end_pos.x:
            v_segments.append((Segment(a, b), 'v'))
        else:
            h_segments.append((Segment(a, b), 'b'))
            h_segments.append((Segment(b, a), 'e'))

        begin_pos = end_pos

    print('------')
    print(end_pos)
    print()

    return h_segments, v_segments


def find_intersections(segments):
    intersections = []

    points = set()

    # print(segments)
    for segment, kind in segments:
        if kind == 'b':
            points.add(segment.begin)
        elif kind == 'e':
            try:
                points.remove(segment.end)
            except KeyError:
                pass
        elif kind == 'v':
            # print(points)
            options = [Point(segment.begin.x, p.y) for p in points if p.y >= segment.begin.y and p.y <= segment.end.y]
            for i in options:
                length = i.manhattan_distance(Point(0, 0))
                if length != 0:
                    intersections.append((i, length))

    return sorted(intersections, key=lambda x: x[1])


def main(inputfile):
    with open(inputfile) as fp:
        path1 = fp.readline().strip().split(',')
        path2 = fp.readline().strip().split(',')

    h_segments1, v_segments1 = walk_path(path1)
    h_segments2, v_segments2 = walk_path(path2)

    segments1 = sorted(h_segments1 + v_segments2, key=lambda x: x[0].begin.x)
    segments2 = sorted(h_segments2 + v_segments1, key=lambda x: x[0].begin.x)

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
    import sys
    main(sys.argv[1])
