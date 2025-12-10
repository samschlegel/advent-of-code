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
    joltage: list[int]

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
        o = z3.Optimize()

        # Create boolean variable for each button (True = press it, False = don't)
        button_vars = z3.BoolVector("button", len(self.buttons))

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
                o.add(xor_result == (target_bit == 1))
            else:
                # No buttons affect this bit, so it must be 0 in target
                o.add(target_bit == 0)

        # Minimize the number of button presses
        # We'll search for solutions with increasing number of presses
        o.minimize(z3.Sum([z3.If(bv, 1, 0) for bv in button_vars]))
        if o.check() == z3.sat:
            m = o.model()
            result = []
            for i, bv in enumerate(button_vars):
                if m.evaluate(bv):
                    result.append(self.buttons[i])
            return tuple(result)

        raise ValueError("No solution found")

    def solve(self):
        return self.solve_z3()

    def solve_z3_p2(self):
        """Solve using Z3 SMT solver.

        We model this as: for each button, how many times do we press it?
        Since XOR is idempotent (x ^ x = 0), we only care about odd/even presses.
        So we use boolean variables: True = press odd times, False = press even times.
        """
        o = z3.Optimize()

        # Create integer variable for each button, where the value is the number of presses
        button_vars = z3.IntVector("button", len(self.buttons))
        # Ensure they're only positive
        for x in button_vars:
            o.add(x >= 0)
        # Minimize the number of button presses
        h = o.minimize(z3.Sum(button_vars))

        # Find the number of bits we need to consider
        max_val = max(self.diagram, max(self.buttons) if self.buttons else 0)
        num_bits = max_val.bit_length() if max_val > 0 else 1

        # For each bit position, constrain that the XOR equals the target
        for bit_pos in range(num_bits):
            target_bit = (self.diagram >> bit_pos) & 1
            target_joltage = self.joltage[bit_pos]

            # Collect which buttons affect this bit
            button_bits: list[z3.ArithRef] = []
            for i, button_val in enumerate(self.buttons):
                button_bit = (button_val >> bit_pos) & 1
                if button_bit == 1:
                    button_bits.append(button_vars[i])

            if button_bits:
                joltage_result = button_bits[0]
                for b in button_bits[1:]:
                    joltage_result += b
                o.add(joltage_result == target_joltage)
            else:
                # No buttons affect this bit, so it must be 0 in target
                o.add(target_bit == 0)

        while o.check() == z3.sat:
            m = o.model()
            result = []
            return m.eval(z3.Sum(button_vars)).as_long()

        raise ValueError("No solution found")


def parse(filename):
    machines = []
    with open(filename, "r") as f:
        for line in f:
            parts = line.split()
            diagram = parse_diagram(parts[0])
            buttons = parse_buttons(parts[1:-1])
            joltage = parse_joltage(parts[-1])
            machine = Machine(diagram, buttons, joltage)
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


def parse_joltage(s):
    return list(map(int, s[1:-1].split(",")))


def part1(filename):
    machines = parse(filename)
    total_presses = 0
    for i, machine in enumerate(machines):
        print(machine)
        solution_z3 = machine.solve_z3()
        solution_bf = machine.solve_bruteforce()
        if len(solution_z3) != len(solution_bf):
            print(
                f"{i+1} Z3 and BF disagree on {machine}: Z3={solution_z3}, BF={solution_bf}"
            )
            return

        # print(solution)
        total_presses += len(solution_bf)
    print(f"Total presses: {total_presses}")


def part2(filename):
    machines = parse(filename)
    total_presses = 0
    for i, machine in enumerate(machines):
        print(machine)
        solution = machine.solve_z3_p2()

        total_presses += solution
    print(f"Total presses: {total_presses}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Day 9")
    parser.add_argument(
        "filename", nargs="?", help="Input file", default="2025/day9/example_input.txt"
    )
    args = parser.parse_args()

    part1(args.filename)
    part2(args.filename)
