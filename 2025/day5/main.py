import argparse


def part1(filename):
    with open(filename, "r") as f:
        ranges = []
        for line in f:
            line = line.strip()
            if line == "":
                break
            start, end = map(int, line.split("-"))
            ranges.append([start, end])
            print(f"Range: {start}-{end}")

        ingredients = []
        for line in f:
            ingredients.append(int(line.strip()))
        ingredients.sort()

        merged_ranges = []
        for start, end in sorted(ranges):
            if not merged_ranges or (merged_ranges[-1][1] < (start - 1)):
                merged_ranges.append([start, end])
            else:
                merged_ranges[-1][1] = max(merged_ranges[-1][1], end)

        print(f"Merged ranges: {merged_ranges}")

        fresh_ingredients = []
        for ingredient in ingredients:
            for start, end in merged_ranges:
                if start <= ingredient <= end:
                    fresh_ingredients.append(ingredient)

        print(f"Fresh ingredients: {len(fresh_ingredients)}")
        possible_fresh = sum(map(lambda x: x[1] - x[0] + 1, merged_ranges))
        print(f"Possible fresh: {possible_fresh}")


def part2(filename):
    pass


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Advent of Code 2025 Day 5")
    parser.add_argument("filename", help="Input file")
    args = parser.parse_args()

    part1(args.filename)
    part2(args.filename)
