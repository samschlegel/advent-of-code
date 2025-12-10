import argparse
from pprint import pprint


def part1(filename):
    with open(filename, "r") as f:
        corners = [tuple(map(int, line.strip().split(","))) for line in f]
    largest = 0
    for i in range(len(corners)):
        a = corners[i]
        for j in range(i + 1, len(corners)):
            b = corners[j]
            size = (abs(a[0] - b[0]) + 1) * (abs(a[1] - b[1]) + 1)
            if size > largest:
                largest = size
    print(largest)


def part2(filename):
    pass


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Day 9")
    parser.add_argument(
        "filename", nargs="?", help="Input file", default="2025/day9/example_input.txt"
    )
    args = parser.parse_args()

    part1(args.filename)
    part2(args.filename)
