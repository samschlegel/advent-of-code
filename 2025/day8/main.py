import argparse
from dataclasses import dataclass
from functools import reduce
from math import sqrt


@dataclass
class Box:
    id: int
    pos: tuple[int, int, int]
    circuit: int


def distance(a: Box, b: Box):
    return sqrt(
        (a.pos[0] - b.pos[0]) ** 2
        + (a.pos[1] - b.pos[1]) ** 2
        + (a.pos[2] - b.pos[2]) ** 2
    )


def connected(a: Box, b: Box):
    return a.circuit == b.circuit


def connect(circuits: dict[int, set[int]], boxes: list[Box], a: Box, b: Box):
    a_circuit = a.circuit
    b_circuit = b.circuit
    if a_circuit != b_circuit:
        circuits[a_circuit].update(circuits[b_circuit])
        for i in circuits[b_circuit]:
            boxes[i].circuit = a_circuit
        del circuits[b_circuit]


def get_closest_pairs_list(boxes) -> list[tuple[int, int, float]]:
    l = []
    for a_i in range(len(boxes)):
        for b_i in range(a_i + 1, len(boxes)):
            a, b = boxes[a_i], boxes[b_i]
            dist = distance(a, b)
            l.append((a, b, dist))
    l.sort(key=lambda x: x[2])
    return l


def part1(filename, connections):
    boxes = []
    circuits = {}
    with open(filename, "r") as f:
        for i, line in enumerate(f):
            boxes.append(Box(i, tuple(map(int, line.strip().split(",", 2))), i))
            circuits[i] = {i}
    closest_pairs = get_closest_pairs_list(boxes)
    for i in range(connections):
        a, b, _ = closest_pairs[i]
        connect(circuits, boxes, a, b)
        print(f"[{i+1}/{connections}] Connected {a} and {b}")

    print(len(circuits))
    biggest_3 = sorted(len(c) for c in circuits.values())[-3:]
    print(reduce(lambda x, y: x * y, biggest_3))


def part2(filename, connections):
    pass


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Day 8")
    parser.add_argument(
        "filename", nargs="?", help="Input file", default="2025/day8/example_input.txt"
    )
    parser.add_argument(
        "connections",
        nargs="?",
        type=int,
        help="Input file",
        default=10,
    )
    args = parser.parse_args()

    part1(args.filename, args.connections)
    part2(args.filename, args.connections)
