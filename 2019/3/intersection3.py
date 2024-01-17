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

    def distance(self, pos):
        return (abs(self.x) - abs(pos.x)) + (abs(self.y) - abs(pos.y))


class Segment:
    def __init__(self, begin, end, direction):
        self.begin = begin
        self.end = end
        self.direction = direction

    def __repr__(self):
        return f"({self.begin.__repr__()}, {self.end.__repr__()}, {self.direction})"

    def __str__(self):
        return repr(self)

    @property
    def normalized_begin(self):
        if self.direction == 'R' or self.direction == 'U':
            return self.begin
        return self.end

    @property
    def normalized_end(self):
        if self.direction == 'R' or self.direction == 'U':
            return self.end
        return self.begin

    @property
    def length(self):
        if self.begin.x == self.end.x:
            return abs(self.end.y) - abs(self.begin.y)
        return abs(self.end.x) - abs(self.begin.x)


def new_segment(direction, begin, length):
    compass = {
        'D': lambda p, v: p + ( 0, -v),
        'L': lambda p, v: p + (-v,  0),
        'R': lambda p, v: p + ( v,  0),
        'U': lambda p, v: p + ( 0,  v),
    }
    end = compass[direction](begin, length)
    return Segment(begin, end, direction)


def walk_path(paths):
    begin_pos = Position(0, 0)

    h_segments = []
    v_segments = []

    for item in paths:
        print(item, begin_pos)
        direction = item[0]
        length = int(item[1:])

        segment = new_segment(direction, begin_pos, length)

        if begin_pos.x == segment.end.x:
            v_segments.append((segment.normalized_begin, segment, 'v'))
        else:
            h_segments.append((segment.normalized_begin, segment, 'b'))
            h_segments.append((segment.normalized_end, segment, 'e'))

        begin_pos = segment.end

    print('------')
    print(begin_pos)
    print()

    return h_segments, v_segments


def find_intersections(segments):
    intersections = []

    points = set()

    print(segments)
    for pos, segment, kind in segments:
        print(points)
        if kind == 'b':
            points.add((pos, segment))
        elif kind == 'e':
            try:
                points.remove((pos, segment))
            except KeyError:
                pass
        elif kind == 'v':
            options = [Position(segment.begin.x, p.y) for p, _ in points if p.y >= segment.begin.y and p.y <= segment.end.y]
            for i in options:
                distance = i.distance(Position(0, 0))
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

    segments1 = sorted(h_segments1 + v_segments2, key=lambda x: x[0].x)
    segments2 = sorted(h_segments2 + v_segments1, key=lambda x: x[0].x)

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
