import sys


def part1():
    filename = sys.argv[1]
    with open(filename, "r") as file:
        dial_pos = 50
        zeros = 0
        for line in file:
            match line[0]:
                case "L":
                    dial_pos = (dial_pos - int(line[1:])) % 100
                case "R":
                    dial_pos = (dial_pos + int(line[1:])) % 100
            if dial_pos == 0:
                zeros += 1
    print(zeros)


def part2():
    filename = sys.argv[1]
    with open(filename, "r") as file:
        dial_pos = 50
        zeros = 0
        print("op\tprev\tnew\tdial\tclicks\tzeros")
        for line in file:
            prev_dial_pos = dial_pos
            match line[0]:
                case "L":
                    new_dial_pos = dial_pos - int(line[1:])
                case "R":
                    new_dial_pos = dial_pos + int(line[1:])
                case _:
                    raise ValueError(f"Invalid direction: {line[0]}")
            if new_dial_pos < 0:
                clicks = abs(new_dial_pos) // 100
                if prev_dial_pos > 0:
                    clicks += 1
            else:
                clicks = abs(new_dial_pos // 100)
                if new_dial_pos == 0:
                    clicks += 1
            zeros += clicks
            dial_pos = new_dial_pos % 100
            print(
                f"{line.strip()}:\t{prev_dial_pos}\t{new_dial_pos}\t{dial_pos}\t{clicks}\t{zeros}"
            )
    print(zeros)


if __name__ == "__main__":
    part1()
    part2()
