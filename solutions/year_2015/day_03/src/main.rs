#![feature(test)]
#![allow(dead_code)]

extern crate test;
use utilities::read_input;

const PATH: &str = "input.txt";

fn main() {
    let input = read_input(PATH);
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2(&input));
}

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn part_1(input: &str) -> usize {
    part_1_v1(&input)
}

fn part_2(input: &str) -> usize {
    part_2_set(&input)
}

fn part_1_v1(input: &str) -> usize {
    let mut grid: HashMap<Point, u32> = HashMap::new();
    let mut santa = Point{ x: 0, y: 0 };

    *grid.entry(santa).or_insert(0) += 1;

    for &b in input.as_bytes() {
        match b {
            b'>' => santa.x += 1,
            b'v' => santa.y += 1,
            b'<' => santa.x -= 1,
            b'^' => santa.y -= 1,
            _ => (), 
        }

        *grid.entry(santa).or_insert(0) += 1;
    }

    grid.len()
}

fn part_2_set(input: &str) -> usize {
    let mut grid: HashSet<Point> = HashSet::new();
    let mut santa = Point{ x: 0, y: 0 };
    let mut robot_santa = Point{ x: 0, y: 0 };

    grid.insert(santa);
    grid.insert(robot_santa);

    for (i, b) in input.bytes().enumerate() {
        let current_santa = if i % 2 == 0 { &mut santa } else { &mut robot_santa };

        match b {
            b'>' => current_santa.x += 1,
            b'v' => current_santa.y += 1,
            b'<' => current_santa.x -= 1,
            b'^' => current_santa.y -= 1,
            _ => (), 
        }

        grid.insert(*current_santa);
    }

    grid.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(">"), 2);
        assert_eq!(part_1("^>v<"), 4);
        assert_eq!(part_1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("^v"), 3);
        assert_eq!(part_2("^>v<"), 3);
        assert_eq!(part_2("^v^v^v^v^v"), 11);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2_set(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_set(&input));
    }
}