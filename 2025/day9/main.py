import argparse
from functools import reduce

import matplotlib.pyplot as plt
import numpy as np
from skimage import io
from skimage.color import rgb2gray
from skimage.segmentation import flood

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

    def convert_point(xy):
        return x_map[xy[0]], y_map[xy[1]]

    # Create image as numpy array (height, width, channels)
    # Note: PIL uses (width, height) but numpy uses (height, width)
    img = np.zeros((res[1], res[0], 3), dtype=np.uint8)
    img[:, :] = INSIDE

    n_corners = len(corners)
    for i in range(n_corners):
        # Draw red corner
        a = convert_point(corners[i])
        # Note: numpy indexing is [row, col] = [y, x]
        img[a[1], a[0]] = CORNER
        # Connect with line
        b = convert_point(corners[(i + 1) % n_corners])
        for x in line(a, b):
            img[x[1], x[0]] = EDGE

    # Because of how we convert points, 0,0 will always be outside, so start a floodfill from there
    # flood only supports grayscale, so first convert our image to that
    grayscale_img = rgb2gray(img)
    mask = flood(grayscale_img, (0, 0), connectivity=1)
    img[mask] = OUTSIDE

    def check_line_np(a, b):
        if a[0] == b[0]:
            min_y = min(a[1], b[1])
            max_y = max(a[1], b[1]) + 1
            return not np.any(mask[min_y:max_y, a[0]])
        elif a[1] == b[1]:
            min_x = min(a[0], b[0])
            max_x = max(a[0], b[0]) + 1
            return not np.any(mask[a[1], min_x:max_x])
        else:
            raise ValueError("Line is not horizontal or vertical")

    # build all the potential rects first and sort by size since checking the lines is costly
    potential_rects = []
    for i in range(n_corners):
        for j in range(i + 1, n_corners):
            a = convert_point(corners[i])
            b = convert_point(corners[j])
            actual_area = area(corners[i], corners[j])
            potential_rects.append((actual_area, a, b))
    potential_rects.sort(key=lambda x: -x[0])

    for actual_area, a, c in potential_rects:
        b = (a[0], c[1])
        d = (c[0], a[1])
        if (
            not check_line_np(a, b)
            or not check_line_np(b, c)
            or not check_line_np(c, d)
            or not check_line_np(d, a)
        ):
            continue
        max_area = actual_area
        break
        # iterate over the lines ab, bc, cd, and da, asserting that all their pixels are non-black
    print(f"Max area: {max_area}")

    # Display the image using matplotlib
    # plt.imshow(img)
    # plt.axis("off")
    # plt.show()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Day 9")
    parser.add_argument(
        "filename", nargs="?", help="Input file", default="2025/day9/example_input.txt"
    )
    args = parser.parse_args()

    part1(args.filename)
    part2(args.filename)
