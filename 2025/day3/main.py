from pathlib import Path
import sys


def max_for_bank(bank):
    highest_to_right = [0 for _ in bank]
    highest_digit = 0
    highest_value = 0
    for i in range(len(bank) - 1, -1, -1):
        highest_to_right[i] = highest_digit
        if highest_digit > 0:
            highest_value = max(highest_value, bank[i] * 10 + highest_digit)
        highest_digit = max(highest_digit, bank[i])

    return highest_value


def part1(filename):
    with open(filename, "r") as f:
        banks = [list(map(int, line.strip())) for line in f.readlines()]
    max_values = [max_for_bank(bank) for bank in banks]
    total_output = sum(max_values)
    print(max_values)
    print(total_output)


def part2(filename):
    pass


if __name__ == "__main__":
    try:
        filename = sys.argv[1]
    except IndexError:
        filename = Path(__file__).parent / "example_input.txt"
    part1(filename)
    part2(filename)
