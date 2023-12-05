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

fn part_1(input: &str) -> usize {
    part_1_v1(&input)
}

fn part_2(input: &str) -> usize {
    part_2_v2(&input)
}

fn part_1_v1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut vowels = 0;
            let mut double = false;
            let mut forbidden = false;
            let chars = line.as_bytes();

            for (i, &c) in chars.iter().enumerate() {
                if i < chars.len() - 1 {
                    let n: u8 = chars[i + 1];

                    forbidden = match c {
                        b'a' if n == b'b' => true,
                        b'c' if n == b'd' => true,
                        b'p' if n == b'q' => true,
                        b'x' if n == b'y' => true,
                        _ => false,
                    };

                    if forbidden { break; }

                    double |= c == n;
                }

                if vowels < 3 { 
                    vowels += match c {
                        b'a' | b'e' | b'i' | b'o' | b'u' => 1,
                        _ => 0,
                    }
                }
            }

            vowels >= 3 && double && !forbidden
        }).count()
}

use std::collections::HashSet;

// Unfinished (read: broken)
fn part_2_v1(input: &str) -> usize {
    input
    .lines()
    .filter(|line| {
        let mut pair = false;
        let mut inbetween = false;
        let chars = line.as_bytes();
        let length = chars.len();
        let mut uniques: HashSet<(u8, u8)> = HashSet::new();

        dbg!(line);

        for (i, &c) in chars.iter().enumerate() {
            if !inbetween && i < length - 3 {
                inbetween |= c == chars[i + 2];
            }

            if !pair && i < length - 1 && c == chars[i + 1] {
                let key: (u8, u8) = (c, chars[i + 1]);
                pair |= !uniques.insert(key) || uniques.contains(&(c, chars[i + 1]));
            }

            if pair && inbetween {
                break;
            }
        }

        pair && inbetween
    }).count()
}

fn part_2_v2(input: &str) -> usize {
    input
        .lines()
        .filter(|s| {
            let mut pair_seen = false;
            let mut in_between_seen = false;

            let bytes = s.bytes().enumerate();

            for (i, c) in bytes.clone() {
                if i >= 2 {
                    let a = &s[i - 2..i];
                    // bitor is faster than assigning
                    pair_seen |= a.contains(&s[i..]);
                }

                if let Some((_, b)) = bytes.clone().nth(i + 2) {
                    // faster than if c == b { in_between_seen = true }
                    in_between_seen |= c == b;
                }
            }

            pair_seen && in_between_seen
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    #[ignore]
    fn test_part_1() {
        assert_eq!(part_1("ugknbfddgicrmopn"), 1);
        assert_eq!(part_1("aaa"), 1);
        assert_eq!(part_1("jchzalrnumimnmhp"), 0);
        assert_eq!(part_1("haegwjzuvuyypxyu"), 0);
        assert_eq!(part_1("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part_2("xxyxx"), 1);
        assert_eq!(part_2("uurcxstgmygtbstg"), 0);
        assert_eq!(part_2("ieodomkazucvgmuy"), 0);
        assert_eq!(part_2("aaaa"), 1);
        assert_eq!(part_2("aaabcb"), 0);
    }

    #[bench]
    #[ignore]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1_v1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_v1(&input));
    }

    #[bench]
    fn bench_part_2_v2(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_v2(&input));
    }
}