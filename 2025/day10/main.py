import argparse
from functools import reduce
import itertools
from dataclasses import dataclass
import operator


@dataclass
class Machine:
    diagram: int
    buttons: list[int]
    cost: None

    def solve_bruteforce(self):
        for depth in range(0, 10):
            for permutation in itertools.permutations(self.buttons, depth):
                if not permutation:
                    continue
                result = reduce(operator.xor, permutation)
                if result == self.diagram:
                    return permutation
        raise ValueError("No solution found")

    def solve(self):
        return self.solve_bruteforce()


def parse(filename):
    machines = []
    with open(filename, "r") as f:
        for line in f:
            parts = line.split()
            diagram = parse_diagram(parts[0])
            buttons = parse_buttons(parts[1:-1])
            machine = Machine(diagram, buttons, None)
            machines.append(machine)
    return machines


def parse_diagram(s):
    d = 0
    for i in range(1, len(s) - 1):
        if s[i] == "#":
            d |= 2 ** (i - 1)
    return d


def parse_buttons(s):
    bs = []
    for bstr in s:
        b = 0
        for l in bstr[1:-1].split(","):
            b |= 2 ** int(l)
        bs.append(b)
    return bs


def part1(filename):
    machines = parse(filename)
    total_presses = 0
    for machine in machines:
        print(machine)
        solution = machine.solve()
        print(solution)
        total_presses += len(solution)
    print(f"Total presses: {total_presses}")


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
