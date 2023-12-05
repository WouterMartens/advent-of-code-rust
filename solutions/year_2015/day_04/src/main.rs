#![feature(test)]
#![allow(dead_code)]

extern crate test;
use md5;

const INPUT: &str = ""; // redacted

fn main() {
    println!("Part 1 answer: {}", part_1(INPUT));
    println!("Part 2 answer: {}", part_2(INPUT));
}

fn part_1(input: &str) -> u128 {
    part_1_v1(&input)
}

fn part_2(input: &str) -> u128 {
    part_2_v1(&input)
}

fn part_1_v1(input: &str) -> u128 {
    for i in 0..u128::MAX {
        let s = input.to_owned() + &format!("{i}");
        let hash = md5::compute(s);
        let hash = format!("{hash:?}");
        if hash.starts_with("00000") {
            return i
        }
    }

    0
}

fn part_2_v1(input: &str) -> u128 {
    for i in 0..u128::MAX {
        let s = input.to_owned() + &format!("{i}");
        let hash = md5::compute(s);
        let hash = format!("{hash:?}");
        if hash.starts_with("000000") {
            return i
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("abcdef"), 609043);
        assert_eq!(part_1("pqrstuv"), 1048970);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(""), 0);
        assert_eq!(part_2(""), 0);
    }

    #[bench]
    #[ignore]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part_1(&INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part_2(&INPUT));
    }
}