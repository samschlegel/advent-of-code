import argparse


def neighbors(grid, x, y):
    for nx in range(max(0, x - 1), min(len(grid), x + 2)):
        for ny in range(max(0, y - 1), min(len(grid[0]), y + 2)):
            if nx != x or ny != y:
                yield (nx, ny)


def part1(filename):
    with open(filename, "r") as f:
        lines = f.readlines()
    grid = [list(line.strip()) for line in lines]

    accessible = []
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            if grid[y][x] == "@":
                num_rolls = 0
                for nx, ny in neighbors(grid, x, y):
                    if grid[ny][nx] == "@":
                        num_rolls += 1
                if num_rolls < 4:
                    accessible.append((x, y))
    print(len(accessible))


def part2(filename):
    with open(filename, "r") as f:
        lines = f.readlines()
    grid = [list(line.strip()) for line in lines]

    removed = 0
    while True:
        accessible = []
        for y in range(len(grid)):
            for x in range(len(grid[0])):
                if grid[y][x] == "@":
                    num_rolls = 0
                    for nx, ny in neighbors(grid, x, y):
                        if grid[ny][nx] == "@":
                            num_rolls += 1
                    if num_rolls < 4:
                        accessible.append((x, y))
        if not accessible:
            break
        for x, y in accessible:
            grid[y][x] = "."
            removed += 1
        accessible = []
    print("Removed:", removed)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Advent of Code 2025 Day 4")
    parser.add_argument("filename", type=str, help="Input file")
    args = parser.parse_args()

    part1(args.filename)
    part2(args.filename)
