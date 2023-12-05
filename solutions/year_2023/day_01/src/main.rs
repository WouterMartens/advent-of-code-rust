#![feature(test)]
#![allow(dead_code)]

extern crate test;

use utilities::read_input;

const PATH: &str = "input.txt";

fn main() {
    let input = read_input(PATH);
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2_v0(&input));
}

fn part_1(input: &str) -> u32 {
    // line.matches(char::is_numeric).collect()
    let mut numbers: Vec<u16> = Vec::new();
    for line in input.lines() {
        let mut chars: [char; 2] = ['0'; 2];
        let mut i = 0;
        for c in line.chars() {
            if c >= '0' && c <= '9' {
                chars[i] = c;
                i = 1;
                break;
            }
        }
        for c in line.chars().rev() {
            if c >= '0' && c <= '9' {
                chars[i] = c;
                break;
            }
        }

        let combined = format!("{}{}", chars[0], chars[1]).to_string();
        let parsed = combined.parse::<u16>().unwrap();
        numbers.push(parsed);
    }
        
    let mut total: u32 = 0;
    for number in numbers {
        total += number as u32;
    }

    total
}

fn part_2_v0(input: &str) -> u32 {
    let map: [&str; 9]  = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut total = 0;
    
    for line in input.lines() {
        let mut result = 0;

        'character_loop: for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                result = (c.to_digit(10).unwrap() * 10) as u8;
                break;
            }

            for n in map {
                if line[i..].starts_with(n) {
                    result = ((map.iter().position(|&s| s == n).unwrap() + 1) * 10) as u8;
                    break 'character_loop;
                }
            }
        }

        'character_loop: for (i, c) in line.chars().rev().enumerate() {
            if c.is_numeric() {
                result += (c.to_digit(10).unwrap()) as u8;
                break;
            }

            for n in map {
                if line[line.len() - i - 1..].starts_with(n) {
                    result += (map.iter().position(|&s| s == n).unwrap() + 1) as u8;
                    break 'character_loop;
                }
            }
        }

        total += result as u32;
    }
    
    total
}

// ChatGPT's implementation of V0
fn part_2_v1(input: &str) -> u32 {
    let map: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut total = 0;

    for line in input.lines() {
        let mut result = 0;
        let mut i_fw = 0;
        let mut i_bw = line.len();

        while i_fw < line.len() || i_bw > 0 {
            if i_fw < line.len() {
                if let Some(digit) = line[i_fw..].chars().next().unwrap().to_digit(10) {
                    result = (digit * 10) as u8;
                    break;
                }

                for n in &map {
                    if line[i_fw..].starts_with(n) {
                        result = ((map.iter().position(|&s| s == *n).unwrap() + 1) * 10) as u8;
                        break;
                    }
                }

                i_fw += 1;
            }

            if i_bw > 0 {
                if let Some(digit) = line[..i_bw].chars().rev().next().unwrap().to_digit(10) {
                    result += digit as u8;
                    break;
                }

                for n in &map {
                    if line[i_bw - n.len()..].starts_with(n) {
                        result += (map.iter().position(|&s| s == *n).unwrap() + 1) as u8;
                        break;
                    }
                }

                i_bw -= 1;
            }
        }

        total += result as u32;
    }

    total
}

fn part_2_v2(input: &str) -> u32 {
    let map: [&str; 9]  = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    
    input.lines().map(|line| {
        let numeric_indices = line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| c.to_digit(10).map(|digit| (i, digit as u32)));

        let string_indices = map.iter().flat_map(|n| {
            line.match_indices(n).map(move |(i, n)| (i, (map.iter().position(|s| s == &n).unwrap_or(1) + 1) as u32))
        });

        let chained_indices: Vec<(usize, _)> = numeric_indices.chain(string_indices).collect();

        let mut result = 0;

        if let Some(&(_, min_value)) = chained_indices.iter().min_by_key(|&&(key, _)| key) {
            result += min_value * 10;
        }

        if let Some(&(_, max_value)) = chained_indices.iter().max_by_key(|&&(key, _)| key) {
            result += max_value;
        }

        result
    }).sum()
}

// ChatGPT's interpretation of V2
fn part_2_v3(input: &str) -> u32 {
    let map: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    input
        .lines()
        .map(|line| {
            let mut result = 0;
            let mut min_value = u32::MAX;
            let mut max_value = 0;

            for (i, c) in line.chars().enumerate() {
                if let Some(digit) = c.to_digit(10) {
                    min_value = min_value.min(digit);
                    max_value = max_value.max(digit);
                }

                for (j, n) in map.iter().enumerate() {
                    if line[i..].starts_with(n) {
                        let value = j as u32 + 1;
                        min_value = min_value.min(value);
                        max_value = max_value.max(value);
                    }
                }
            }

            if min_value != u32::MAX {
                result += min_value * 10;
                // println!("Value for the lowest key: {}", result);
            }

            if max_value != 0 {
                result += max_value;
                // println!("Value for the highest key: {}", result);
            }

            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    #[ignore]
    fn test_part_1() {
        assert_eq!(part_1(""), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_v0(&read_input(PATH)), 54875);
        assert_eq!(part_2_v1(&read_input(PATH)), 54875);
        assert_eq!(part_2_v2(&read_input(PATH)), 54875);
        assert_eq!(part_2_v3(&read_input(PATH)), 54875);
    }

    #[bench]
    #[ignore]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_v0(&input));
    }

    #[bench]
    fn bench_part_2_v1(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_v1(&input));
    }

    #[bench]
    fn bench_part_2_v2(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_v2(&input));
    }

    #[bench]
    fn bench_part_2_v3(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_v3(&input));
    }
}