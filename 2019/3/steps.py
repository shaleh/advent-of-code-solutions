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
    def __init__(self, begin, end, cost):
        self.begin = begin
        self.end = end
        self.cost = cost

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
    travelled = 0
    end_pos = None

    h_segments = []
    v_segments = []

    for item in paths:
        print(item, begin_pos)
        direction = item[0]
        length = int(item[1:])

        end_pos = compass[direction](begin_pos, length)
        segment = Segment(begin_pos, end_pos, travelled)

        if begin_pos.x == end_pos.x:
            v_segments.append((segment, 'v'))
        else:
            h_segments.append((segment, 'b'))
            h_segments.append((Segment(segment.end, segment.begin, travelled), 'e'))

        begin_pos = end_pos
        travelled += length

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
            if segment.end < segment.begin:
                segment = Segment(segment.end, segment.begin, segment.cost)
            points.add((segment.begin, segment))
        elif kind == 'e':
            if segment.begin < segment.end:
                segment = Segment(segment.end, segment.begin, segment.cost)
            try:
                points.remove((segment.end, segment))
            except KeyError:
                pass
        elif kind == 'v':
            if segment.end < segment.begin:
                segment = Segment(segment.end, segment.begin, segment.cost)
            # print(points)
            for p, other_segment in points:
                if p.y >= segment.begin.y and p.y <= segment.end.y:
                    point = Position(segment.begin.x, p.y)
                    distance = point.distance(Position(0, 0))
                    if distance != 0:
                        cost = (other_segment.cost + point.distance(other_segment.begin)) + (segment.cost + point.distance(segment.begin))
                        intersections.append((point, distance, cost))

    return intersections


def best_choice(items, key):
    return sorted(items, key=key)[0]


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
    intersections = list(set(intersections1 + intersections2))
    if intersections:
        # By Manhattan Distance
        print(best_choice(intersections, lambda x: x[1]))
        # By length
        print(best_choice(intersections, lambda x: x[2]))
    else:
        print("No intersections found.")


if __name__ == '__main__':
    main()
