import argparse
from ast import In


def safe_idx(l, idx, default=None):
    try:
        return l[idx]
    except IndexError:
        return default


def part1(filename):
    with open(filename, "r") as f:
        lines = [list(l.strip()) for l in f]
    cols = len(lines[0])
    splits = 0
    for i in range(1, len(lines)):
        prev = lines[i - 1]
        curr = lines[i]
        for j in range(0, cols):
            if curr[j] == ".":
                if prev[j] in ("|", "S"):
                    curr[j] = "|"
                    continue
            elif curr[j] == "^":
                if prev[j] == "|":
                    try:
                        curr[j - 1] = "|"
                    except IndexError:
                        pass
                    try:
                        curr[j + 1] = "|"
                    except IndexError:
                        pass
                    splits += 1
    for line in lines:
        print("".join(line))
    print(splits)


def part2(filename):
    pass


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Day 7")
    parser.add_argument(
        "filename", nargs="?", help="Input file", default="2025/day7/example_input.txt"
    )
    args = parser.parse_args()

    part1(args.filename)
    part2(args.filename)
