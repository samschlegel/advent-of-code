import argparse
from functools import reduce
import itertools
from dataclasses import dataclass
import operator
import timeit

import z3


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

    def solve_z3(self):
        """Solve using Z3 SMT solver.

        We model this as: for each button, how many times do we press it?
        Since XOR is idempotent (x ^ x = 0), we only care about odd/even presses.
        So we use boolean variables: True = press odd times, False = press even times.
        """
        s = z3.Solver()

        # Create boolean variable for each button (True = press it, False = don't)
        button_vars = [z3.Bool(f"button_{i}") for i in range(len(self.buttons))]

        # Find the number of bits we need to consider
        max_val = max(self.diagram, max(self.buttons) if self.buttons else 0)
        num_bits = max_val.bit_length() if max_val > 0 else 1

        # For each bit position, constrain that the XOR equals the target
        for bit_pos in range(num_bits):
            target_bit = (self.diagram >> bit_pos) & 1

            # Collect which buttons affect this bit
            button_bits = []
            for i, button_val in enumerate(self.buttons):
                button_bit = (button_val >> bit_pos) & 1
                if button_bit == 1:
                    button_bits.append(button_vars[i])

            # XOR of booleans: odd number of True values = True, even = False
            if button_bits:
                xor_result = button_bits[0]
                for b in button_bits[1:]:
                    xor_result = z3.Xor(xor_result, b)
                s.add(xor_result == (target_bit == 1))
            else:
                # No buttons affect this bit, so it must be 0 in target
                s.add(target_bit == 0)

        # Minimize the number of button presses
        # We'll search for solutions with increasing number of presses
        for max_presses in range(len(self.buttons) + 1):
            s.push()
            # Add constraint on total number of presses
            # s.add(z3.Sum([z3.If(bv, 1, 0) for bv in button_vars]) <= max_presses)

            if s.check() == z3.sat:
                m = s.model()
                result = []
                for i, bv in enumerate(button_vars):
                    if m.evaluate(bv):
                        result.append(self.buttons[i])
                s.pop()
                return tuple(result)
            s.pop()

        raise ValueError("No solution found")

    def solve(self):
        return self.solve_z3()


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
    total_z3_time = 0
    total_bf_time = 0
    z3_won = 0
    bf_won = 0
    for machine in machines:
        # print(machine)
        t_z3 = timeit.timeit(lambda: machine.solve_z3(), number=10)
        total_z3_time += t_z3
        t_bf = timeit.timeit(lambda: machine.solve_bruteforce(), number=10)
        total_bf_time += t_bf
        solution_z3 = machine.solve_z3()
        solution_bf = machine.solve_bruteforce()
        if solution_z3 != solution_bf:
            print(
                f"Z3 and BF disagree on {machine}: Z3={solution_z3}, BF={solution_bf}"
            )
        elif t_z3 < t_bf:
            z3_won += 1
            print(f"Z3 won: {len(solution_z3)} presses")
        else:
            bf_won += 1
            print(f"BF won: {len(solution_bf)} presses")

        # print(solution)
        total_presses += len(solution_bf)
    print(f"Total presses: {total_presses}")
    print(f"Total Z3 time: {total_z3_time:.6f} seconds")
    print(f"Total BF time: {total_bf_time:.6f} seconds")
    print(f"Z3 won: {z3_won}")
    print(f"BF won: {bf_won}")


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
