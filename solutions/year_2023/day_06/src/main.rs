#![feature(test)]
#![allow(dead_code)]

extern crate test;

use utilities::{isqrt, read_input};

const PATH: &str = "input.txt";
const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

struct Race {
    time: usize,
    distance: usize,
}

fn main() {
    let input = read_input(PATH);
    // let input = TEST_INPUT;
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2_quadratic(&input));
}

// Solve the quadratic equation

fn parse_part_1(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect::<Vec<_>>();
            parts[1..]
                .iter()
                .filter_map(|val| val.trim().parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn parse_part_2(input: &str) -> Race {
    let data = parse_part_1(input)
        .iter()
        .map(|line| {
            line.iter()
                .map(|kernel| kernel.to_string())
                .collect::<String>()
                .parse::<usize>()
                .expect("Should be a valid number")
        })
        .collect::<Vec<_>>();

    Race {
        time: data[0],
        distance: data[1],
    }
}

fn part_1(input: &str) -> usize {
    let data = parse_part_1(input);

    (0..data[0].len())
        .map(|race| {
            let time = data[0][race];
            let distance = data[1][race];

            (1..=time)
                .filter(|&step| (time - step) * step > distance)
                .count()
        })
        .product()
}

fn part_2_naive(input: &str) -> usize {
    let race = parse_part_2(input);
    let time_is_even = (race.time % 2 == 0) as usize;

    let steps = race.time / 2 - time_is_even;

    (1..=steps)
        .filter(|&step| (race.time - step) * step > race.distance)
        .count()
        * 2
        + time_is_even
}

fn part_2_sequential(input: &str) -> usize {
    let race = parse_part_2(input);

    let time_is_even = (race.time % 2 == 0) as usize;
    let mut step = race.time / 2 - time_is_even;

    while step > 0 {
        if (race.time - step) * step <= race.distance {
            break;
        }

        step -= 1;
    }

    (race.time / 2 - step - time_is_even) * 2 + time_is_even
}

fn part_2_binary_search(input: &str) -> usize {
    let race = parse_part_2(input);

    let time_is_even = (race.time % 2 == 0) as usize;
    let race_time = race.time / 2 - time_is_even;

    let mut left = 0;
    let mut right = race_time;

    while left <= right {
        let mid = left + (right - left) / 2;

        if (race.time - mid) * mid > race.distance {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    (race_time - right) * 2 + time_is_even
}

fn part_2_quadratic(input: &str) -> isize {
    let race = parse_part_2(input);

    // -step ^ 2 + time * step - race.distance > 0
    let a = -1 as isize;
    let b = race.time as isize;
    let c = -1 * race.distance as isize;

    let discriminant = b * b - 4 * a * c;
    let root = isqrt(discriminant);
    let denominator = 2 * a;

    let first = (-b + root) / denominator;
    let second = (-b - root) / denominator;

    second - first
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_naive(TEST_INPUT), 71503);
        assert_eq!(part_2_sequential(TEST_INPUT), 71503);
        assert_eq!(part_2_binary_search(TEST_INPUT), 71503);
        assert_eq!(part_2_quadratic(TEST_INPUT), 71503);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2_naive(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_naive(&input));
    }

    #[bench]
    fn bench_part_2_sequential(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_sequential(&input));
    }

    #[bench]
    fn bench_part_2_binary_search(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_binary_search(&input));
    }

    #[bench]
    fn bench_part_2_quadratic(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_quadratic(&input));
    }
}
