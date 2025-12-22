# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is an Advent of Code solutions repository containing solutions in multiple languages (Python, Rust, Julia, TypeScript).

## Project Structure

- `{year}/day{N}/` - Each day's solution with `main.py`, `input.txt`, `example_input.txt`, and optional `test.py`
- `2021/rust/` - Rust workspace with solutions in `src/bin/day{N}.rs`
- `2021/inputs/` - Input files for 2021 Rust solutions

## Commands

### Python (2025 solutions)

```bash
# Run a day's solution
python 2025/day{N}/main.py 2025/day{N}/input.txt

# Run with example input
python 2025/day{N}/main.py 2025/day{N}/example_input.txt

# Run tests for a day
pytest 2025/day{N}/test.py

# Install dependencies
uv sync
```

### Rust (2021 solutions)

```bash
# Run a day's solution
cargo run --bin day{N} --manifest-path 2021/rust/Cargo.toml

# Run benchmarks
cargo bench --manifest-path 2021/rust/Cargo.toml
```

### Julia

```bash
julia 2025/day{N}/jl/main.jl
```

## Python Dependencies

Key libraries available: `z3-solver`, `scipy`, `scikit-image`, `matplotlib`, `pillow`

Python version: 3.14
