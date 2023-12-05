# Advent of Code with Rust
Attempting to learn [Rust](https://doc.rust-lang.org/beta/std/index.html) by solving the [Advent of Code](https://adventofcode.com/) (AoC) problems, creating [unnecessary templates](#copy_template), and abusing [ChatGPT](https://chat.openai.com/). 🥸🦦

## Table of Contents
1. [copy_template](#copy_template)
2. [day_template](#day_template)
3. [solutions](#solutions)
4. [utilities](#utilities)

## copy_template
Contains a program that can copy `day_template` into the `solutions` folder (or elsewhere).

### Usage
```
cd copy_template
cargo run --release -- -h
```
```
Options:
  -s, --source <SOURCE>  Template source folder [default: ../day_template/]
  -t, --target <TARGET>  Destination folder (takes precedence over --year)
  -y, --year <YEAR>      Year of the problem [default: 2015]
  -d, --day <DAY>        Day of the problem, omit to automatically find [default: 0]
  -h, --help             Print help
```
### Examples
The following instructions copy the template to `../solutions/year_2022/day_05`.
```
cargo run --release -- --source ../day_template/ --target ../solutions/year_2022/ --day 5
cargo run --release -- --source ../day_template/ --year 2022 --day 5
cargo run --release -- -y 2022 -d 5
```
The default instructions copy the template to `../solutions/year_2015/day_01`. If `day_01` already exists, it will copy to `day_02` instead, and so on. The default values can be changed in `struct Args` of `/copy_template/src/main.rs`.
```
cargo run --release
```
**Note**: `--release` can be omited. 

**Note**: when both `--target` and `--year` are given, the target flag takes precedence.

## day_template
Contains a template that can be used to solve one day of AoC. The template provides a text file for the input, functions for the solutions of part 1 and part 2, unit tests, and benchmarks. 

## solutions
Contains my solutions per year per day using the Rust language. A source file might contain more than one solution and often different solutions are compared by performance using benchmarks.

## utilities
Contains a library crate that contains useful functions that can be used from within a copied template by default.
