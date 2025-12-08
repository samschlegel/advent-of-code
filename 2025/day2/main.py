import math
from pathlib import Path
import sys


def part1(filename):
    with open(filename, "r") as f:
        ranges = f.read().split(",")
        invalids = []
        for r in ranges:
            first, last = map(int, r.strip().split("-"))
            new_invalids = []
            for i in range(first, last + 1):
                l = math.log10(i)
                if int(l) % 2 != 1:
                    continue
                m = 10 ** ((int(l) + 1) // 2)
                if i % m == i // m:
                    new_invalids.append(i)
            invalids.extend(new_invalids)
            print(f"Range: {first}-{last}, Invalids: {new_invalids}")
    print(sum(invalids))


def part2(filename):
    pass


if __name__ == "__main__":
    try:
        filename = sys.argv[1]
    except IndexError:
        filename = Path(__file__).parent / "example_input.txt"
    part1(filename)
    part2(filename)
