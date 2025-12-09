import argparse
from functools import reduce


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
    pass


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Day 6")
    parser.add_argument("filename", help="Input file")
    args = parser.parse_args()

    part1(args.filename)
    part2(args.filename)
