#!/usr/bin/env python3

class Position:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __repr__(self):
        return f"({self.x}, {self.y})"

    def __str__(self):
        return f"({self.x}, {self.y})"

    def __add__(self, delta):
        return Position(self.x + delta[0], self.y + delta[1])

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

    def __repr__(self):
        return f"({self.begin.__repr__()}, {self.end.__repr__()})"

    def __str__(self):
        return repr(self)

    @property
    def length(self):
        if self.begin.x == self.end.x:
            return self.end.y - self.begin.y
        return self.end.x - self.begin.x


compass = {
    'D': lambda p, v: p + ( 0, -v),
    'L': lambda p, v: p + (-v,  0),
    'R': lambda p, v: p + ( v,  0),
    'U': lambda p, v: p + ( 0,  v),
}


def walk_path(paths):
    begin_pos = Position(0, 0)
    end_pos = None

    h_segments = []
    v_segments = []

    for item in paths:
        print(item, begin_pos)
        direction = item[0]
        length = int(item[1:])

        end_pos = compass[direction](begin_pos, length)
        segment = Segment(*sorted([begin_pos, end_pos]))

        if begin_pos.x == end_pos.x:
            v_segments.append((segment, 'v'))
        else:
            h_segments.append((segment, 'b'))
            h_segments.append((Segment(segment.end, segment.begin), 'e'))

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
            begin, end = (segment.begin, segment.end)
            options = [Position(begin.x, p.y) for p in points if p.y >= begin.y and p.y <= end.y]
            for i in options:
                distance = abs(i.x) + abs(i.y)
                if distance != 0:
                    intersections.append((i, distance))

    return sorted(intersections, key=lambda x: x[1])


def main():
    import sys
    with open(sys.argv[1]) as fp:
        path1 = fp.readline().strip().split(',')
        path2 = fp.readline().strip().split(',')

    h_segments1, v_segments1 = walk_path(path1)
    h_segments2, v_segments2 = walk_path(path2)

    segments1 = sorted(h_segments1 + v_segments2, key=lambda x: x[0].begin.x)
    segments2 = sorted(h_segments2 + v_segments1, key=lambda x: x[0].begin.x)

    intersections1 = find_intersections(segments1)
    intersections2 = find_intersections(segments2)

    # print(intersections1)
    # print(intersections2)

    shortest = None
    if intersections1:
        shortest = intersections1[0]
        if intersections2:
            if intersections2[0][1] < intersections1[0][1]:
                shortest = intersections2[0]
    elif intersections2:
        shortest = intersections2[0]

    if shortest:
        closest, distance = shortest
        print(closest, distance)
    else:
        print("No intersections found.")


if __name__ == '__main__':
    main()
