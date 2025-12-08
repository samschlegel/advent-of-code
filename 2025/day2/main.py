import math
import sys
from functools import lru_cache
from pathlib import Path


@lru_cache(maxsize=None)
def divisors(num_digits):
    divisors = []
    for e in range(1, num_digits // 2 + 1):
        if num_digits % e != 0:
            continue
        a = 0
        b = 0
        while b < 10**num_digits:
            a = b
            b = a * 10**e
            b += 1
        divisors.append(a)
    return divisors


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
    with open(filename, "r") as f:
        ranges = f.read().split(",")
        invalids = set()
        for r in ranges:
            first, last = map(int, r.strip().split("-"))
            new_invalids = set()
            for i in range(first, last + 1):
                num_digits = int(math.log10(i)) + 1
                for d in divisors(num_digits):
                    if i % d == 0:
                        new_invalids.add(i)
            invalids.update(new_invalids)
            print(f"Range: {first}-{last}, Invalids: {sorted(new_invalids)}")
    print(sum(invalids))


if __name__ == "__main__":
    try:
        filename = sys.argv[1]
    except IndexError:
        filename = Path(__file__).parent / "input.txt"
    part1(filename)
    part2(filename)
