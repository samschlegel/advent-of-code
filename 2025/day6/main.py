import argparse
from functools import reduce
from socket import SocketType


def part1(filename):
    with open(filename) as f:
        numbers = []
        eof = False
        for line in f:
            if eof:
                break
            row = line.split()
            if not numbers:
                numbers = [[] for _ in range(len(row))]
            for i, x in enumerate(row):
                if x == "*":
                    eof = True
                    numbers[i] = reduce(lambda a, b: a * b, numbers[i])
                elif x == "+":
                    eof = True
                    numbers[i] = sum(numbers[i])
                else:
                    numbers[i].append(int(x))
    print(sum(numbers))


def part2(filename):
    with open(filename) as f:
        lines = f.readlines()

    cols = max(len(line) for line in lines)
    rows = len(lines)
    numbers = []
    solutions = []
    for x in range(cols - 1, -1, -1):
        num = 0
        for y in range(rows):
            try:
                value = lines[y][x]
            except IndexError:
                value = " "
            if value in (" ", "\n"):
                continue
            if value == "+":
                numbers.append(num)
                solutions.append(sum(numbers))
                num = 0
                numbers = []
            elif value == "*":
                numbers.append(num)
                solutions.append(reduce(lambda a, b: a * b, numbers))
                num = 0
                numbers = []
            else:
                num = num * 10 + int(value)
        if num != 0:
            numbers.append(num)
    print(sum(solutions))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Day 6")
    parser.add_argument(
        "filename", nargs="?", help="Input file", default="2025/day6/example_input.txt"
    )
    args = parser.parse_args()

    part1(args.filename)
    part2(args.filename)
