#![feature(test)]
#![allow(dead_code)]

extern crate test;

use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};

use utilities::read_input;

const PATH: &str = "input.txt";

const TEST_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

const TEST_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const TEST_INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

fn parse(input: &str) -> (VecDeque<usize>, HashMap<&str, [&str; 2]>) {
    let instructions: VecDeque<_> = input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|x| match x {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid direction instruction"),
        })
        .collect();

    let nodes: HashMap<&str, [&str; 2]> = input
        .lines()
        .skip(2)
        .map(|line| {
            let (location, move_to) = line.split_once(" = ").unwrap();
            let move_to: [&str; 2] = [&move_to[1..4], &move_to[6..9]];
            (location, move_to)
        })
        .collect();

    (instructions, nodes)
}

fn main() {
    let input = read_input(PATH);
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let (mut instructions, nodes) = parse(input);

    let mut arrived = false;

    let mut current_position = "AAA";
    let mut count = 0;

    while !arrived {
        let instruction = instructions.pop_front().expect("Should exist");
        // Since we always push it to the back instantly
        instructions.push_back(instruction);

        current_position = nodes.get(current_position).unwrap()[instruction];

        count += 1;

        if current_position == "ZZZ" {
            arrived = true;
        }
    }

    count
}

fn part_2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    #[ignore]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT_1), 2);
        assert_eq!(part_1(TEST_INPUT_2), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_naive(TEST_INPUT_3), 6);
        assert_eq!(part_2_graphs(TEST_INPUT_3), 6);
    }

    #[bench]
    #[ignore]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1(&input));
    }

    #[bench]
    #[ignore]
    fn bench_part_2(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2(&input));
    }
}
