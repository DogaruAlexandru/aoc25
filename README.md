# Advent of Code 2025 Solutions

This repository contains my solutions for the [Advent of Code 2025](https://adventofcode.com/2025) puzzles. Each day's challenge is organized in its own directory (e.g., `day1`, `day2`, ...), with Rust code.

## Structure

- `dayX/` — Directory for each day's puzzle, containing:
  - `src/` — Rust source code for the solution
  - `input.txt` — Puzzle input (not tracked in git, see below)
  - `Cargo.toml` — Rust project configuration

## Running Solutions

To run a solution for a specific day, navigate to that day's directory and use Cargo:

```sh
cd day1
cargo run --release
```

## Note about `input.txt`

The `input.txt` files are **not included in the repository** (see `.gitignore`).  
To run a solution, you must create the appropriate `input.txt` file in each day's directory with your puzzle input.
