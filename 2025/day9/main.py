import argparse
from functools import reduce
import math
from pprint import pprint

from PIL import Image, ImageDraw

OUTSIDE = (255, 255, 255)
INSIDE = (0, 0, 0)
CORNER = (255, 0, 0)
EDGE = (0, 255, 0)


def part1(filename):
    with open(filename, "r") as f:
        corners = [tuple(map(int, line.strip().split(","))) for line in f]
    largest = 0
    for i in range(len(corners)):
        a = corners[i]
        for j in range(i + 1, len(corners)):
            b = corners[j]
            size = area(a, b)
            if size > largest:
                largest = size
    print(largest)


def sign(x):
    if x > 0:
        return 1
    elif x < 0:
        return -1
    else:
        return 0


def line(a, b, include_ends=False):
    # iterate over all the points in the line formed by a and b.
    # the line is expected to be either strictly horizontal, or strictly vertical
    if not (a[0] == b[0] or a[1] == b[1]):
        raise ValueError("Line must be horizontal or vertical")
    if include_ends:
        yield a
    d = (sign(b[0] - a[0]), sign(b[1] - a[1]))
    x = a
    while True:
        x = (x[0] + d[0], x[1] + d[1])
        if x == b:
            break
        yield x
    if include_ends:
        yield b


def area(a, b):
    return (abs(a[0] - b[0]) + 1) * (abs(a[1] - b[1]) + 1)


def part2(filename):
    with open(filename, "r") as f:
        corners = [tuple(map(int, line.strip().split(","))) for line in f]
    res = reduce(
        lambda acc, x: (max(acc[0], x[0] + 1), max(acc[1], x[1] + 1)), corners, (0, 0)
    )
    xs = set()
    ys = set()
    for c in corners:
        xs.add(c[0])
        ys.add(c[1])
    xs = sorted(xs)
    ys = sorted(ys)
    x_map = {}
    for i in range(len(xs)):
        x_map[xs[i]] = 2 * i + 1
    y_map = {}
    for i in range(len(ys)):
        y_map[ys[i]] = 2 * i + 1
    res = (len(xs) * 2 + 1, len(ys) * 2 + 1)
    print(f"res: {res}")

    def convert_point(xy):
        return x_map[xy[0]], y_map[xy[1]]

    img = Image.new("RGB", res, color=INSIDE)
    n_corners = len(corners)
    for i in range(n_corners):
        # Draw red corner
        a = convert_point(corners[i])
        img.putpixel(a, CORNER)
        # Connect with line
        b = convert_point(corners[(i + 1) % n_corners])
        for x in line(a, b):
            img.putpixel(x, EDGE)

    # Because of how we convert points, 0,0 will always be outside, so start a floodfill from there
    ImageDraw.floodfill(img, (0, 0), OUTSIDE)

    def check_line(a, b):
        for x in line(a, b, include_ends=True):
            if img.getpixel(x) == OUTSIDE:
                return False
        return True

    max_area = 0
    total_rects = n_corners * (n_corners - 1) // 2
    valid_rects = 0
    for i in range(n_corners):
        for j in range(i + 1, n_corners):
            a = convert_point(corners[i])
            c = convert_point(corners[j])
            b = (a[0], c[1])
            d = (c[0], a[1])
            if (
                not check_line(a, b)
                or not check_line(b, c)
                or not check_line(c, d)
                or not check_line(d, a)
            ):
                continue
            valid_rects += 1
            actual_area = area(corners[i], corners[j])
            max_area = max(max_area, actual_area)
            # iterate over the lines ab, bc, cd, and da, asserting that all their pixels are non-black
    print(f"Total rectangles: {total_rects}")
    print(f"Valid rectangles: {valid_rects}")
    print(f"Max area: {max_area}")

    img.show()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Day 9")
    parser.add_argument(
        "filename", nargs="?", help="Input file", default="2025/day9/example_input.txt"
    )
    args = parser.parse_args()

    part1(args.filename)
    part2(args.filename)
