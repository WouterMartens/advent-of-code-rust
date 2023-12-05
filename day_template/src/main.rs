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

fn part_1(input: &str) -> u32 {
    input.lines();
    0
}

fn part_2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(""), 0);
        assert_eq!(part_1(""), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(""), 0);
        assert_eq!(part_2(""), 0);
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