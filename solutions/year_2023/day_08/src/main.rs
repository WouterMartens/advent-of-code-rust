#![feature(test)]
#![allow(dead_code)]

extern crate test;

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

fn part_1(input: &str) -> usize {
    let (instructions, nodes) = parse(input);

    let mut arrived = false;
    let mut current_position = "AAA";
    let mut count = 0;

    while !arrived {
        current_position =
            nodes.get(current_position).unwrap()[instructions[count % instructions.len()]];

        count += 1;

        if current_position == "ZZZ" {
            arrived = true;
        }
    }

    count
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn part_2(input: &str) -> u64 {
    let (instructions, nodes) = parse(input);

    nodes
        .iter()
        .filter(|&(node, _)| node.ends_with('A'))
        .map(|start_node| {
            let mut current_position = start_node;

            (0..)
                .find(|i| {
                    current_position = nodes
                        .get_key_value(&current_position.1[instructions[*i % instructions.len()]])
                        .unwrap();
                    current_position.0.ends_with('Z')
                })
                .unwrap()
                + 1
        })
        .fold(1, |acc, x| lcm(acc, x as u64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT_1), 2);
        assert_eq!(part_1(TEST_INPUT_2), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT_3), 6);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2(&input));
    }
}
