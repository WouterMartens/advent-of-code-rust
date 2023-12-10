#![feature(test)]
#![allow(dead_code)]

extern crate test;
use utilities::read_input;

const PATH: &str = "input.txt";

const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

fn main() {
    let input = read_input(PATH);
    println!("Part 1 answer: {}", solve(&input, false));
    println!("Part 2 answer: {}", solve(&input, true));
}

fn solve(input: &str, reverse: bool) -> i64 {
    let mut result = 0;

    for line in input.lines() {
        let mut temp: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        if reverse {
            temp.reverse()
        }

        while temp.iter().any(|&x| x != 0) {
            result += *temp.last().unwrap();
            temp = temp.windows(2).map(|pair| pair[1] - pair[0]).collect();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(solve("0 3 6 9 12 15", false), 18);
        assert_eq!(solve("1 3 6 10 15 21", false), 28);
        assert_eq!(solve("10 13 16 21 30 45", false), 68);
        assert_eq!(solve(TEST_INPUT, false), 114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve("0 3 6 9 12 15", true), -3);
        assert_eq!(solve("1 3 6 10 15 21", true), 0);
        assert_eq!(solve("10 13 16 21 30 45", true), 5);
        assert_eq!(solve(TEST_INPUT, true), 2);
    }

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| solve(&input, true));
    }
}
