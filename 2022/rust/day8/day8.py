#!/usr/bin/env python3


def go_left(values):
    return reversed(list(enumerate(values)))


def go_right(values):
    return enumerate(values)


def next_greater(move, arr):
    res = [None for i in arr]
    stack = []
    for i, num in move(arr):
        # <= instead of < since tree of equal height is considered "greater"
        while stack and arr[stack[-1]] <= num:
            index = stack.pop()
            res[index] = num
        stack.append(i)
    return res


def next_greater_right(arr):
    return next_greater(go_right, arr)


def next_greater_left(arr):
    return next_greater(go_left, arr)


def transpose(rows):
    w = len(rows[0])
    h = len(rows)
    return [[rows[i][j] for i in range(h)] for j in range(w)]


def part1(forest):
    transposed_forest = transpose(forest)

    blocking_trees_right = [next_greater_right(row) for row in forest]
    blocking_trees_left = [next_greater_left(row) for row in forest]
    blocking_trees_above = transpose(
        [next_greater_left(row) for row in transposed_forest]
    )
    blocking_trees_below = transpose(
        [next_greater_right(row) for row in transposed_forest]
    )

    visible_trees = {}
    for i in range(len(forest)):
        for j in range(len(forest[i])):
            if (
                blocking_trees_left[i][j] is None
                or blocking_trees_right[i][j] is None
                or blocking_trees_above[i][j] is None
                or blocking_trees_below[i][j] is None
            ):
                visible_trees[(i, j)] = forest[i][j]

    print(len(visible_trees))


if __name__ == "__main__":
    with open("../../inputs/8") as f:
        forest = [[int(c) for c in row.strip()] for row in f.readlines()]

    part1(forest)
