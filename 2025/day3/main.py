import argparse
from pathlib import Path
import sys


def max_for_bank(bank, num_digits=2):
    value = 0
    min_idx = 0
    for remaining_digits in range(num_digits, 0, -1):
        max_idx = len(bank) - remaining_digits
        next_digit, next_idx = 0, -1
        for i in range(min_idx, max_idx + 1):
            if bank[i] > next_digit:
                next_digit, next_idx = bank[i], i
        value = value * 10 + next_digit
        min_idx = next_idx + 1

    return value


def part1(filename, num_digits=2):
    with open(filename, "r") as f:
        banks = [list(map(int, line.strip())) for line in f.readlines()]
    max_values = [max_for_bank(bank, num_digits) for bank in banks]
    total_output = sum(max_values)
    print(max_values)
    print(total_output)


def part2(filename):
    part1(filename, num_digits=12)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Advent of Code 2025 Day 3")
    parser.add_argument("filename", type=str, help="Input file")
    args = parser.parse_args()

    part1(args.filename)
    part2(args.filename)
