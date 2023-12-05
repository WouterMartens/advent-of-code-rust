#![feature(test)]
#![allow(dead_code)]
#![feature(slice_group_by)]

extern crate test;
use utilities::read_input;

const PATH: &str = "input.txt";
const TEST_INPUT: &str =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

fn main() {
    let input = read_input(PATH);
    // let input = TEST_INPUT;
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2(&input));
}

// Checking for numbers not touching a special character
// This avoids edge cases where two numbers touch the same character 
// Me from the future: ;-;
fn part_1(input: &str) -> usize {
    let max_x = input.lines().nth(0).unwrap().len() - 1;
    let max_y = input.lines().count() - 1;
    let mut parsed_input = String::new();

    for (row, line) in input.lines().enumerate() {
        let digit_indices: Vec<_> = line
            .match_indices(|c: char| c.is_digit(10))
            .map(|(i, _)| i)
            .collect();
        let groups: Vec<&[usize]> = digit_indices
            .group_by(|&a, &b| b < a + 2)
            .collect();
        let mut parsed_line: String = line.into();

        for group in groups { // into.iter().take(2)
            let r1 = row.saturating_sub(1);
            let c1 = group.iter().min().unwrap().saturating_sub(1);

            let r2 = (row + 1).min(max_x);
            let c2  = (group.iter().max().unwrap() + 1).min(max_y);

            let mut special_character_in_slices = false;

            for r in r1..=r2 {
                let slice = &input.lines().nth(r).unwrap()[c1..=c2];
                special_character_in_slices |= !slice
                    .chars()
                    .map(|c| c.is_digit(10) || c == '.')
                    .all(|b| b == true);
            }

            if !special_character_in_slices {
                for col in group {
                    parsed_line.replace_range(col..=col, ".")
                }
            }
        }

        parsed_input.push_str(&parsed_line);
        parsed_input.push('\n');
    }

    println!("{parsed_input}");
        
    parsed_input
        .lines().map(|line| {
            let line = line
            .char_indices()
            .filter_map(|(i, c)|
                c.to_digit(10).map(|d| (i, d))
            )
            .collect::<Vec<(usize, u32)>>();
        
            // List with (index, digit) tuples
            line
                // Create groups with digits of seperate numbers
                .group_by(|&a, &b| b.0 < a.0 + 2)
                // Map tuple to digits only
                .map(|e| e.iter().map(|(_, d)| d).collect::<Vec<_>>())
                // Build numbers from digits
                .map(|nums| {
                    // Iterate digits in reverse, multiply and sum to get to number value
                    // (first digit * 1, second * 10, etc.)
                    nums.iter().rev().enumerate().map(move |(i, &n)| {
                        let m = 10usize.pow(i as u32);
                        (*n as usize) * m
                    }).sum::<usize>()
                })
                // .inspect(|a| println!("{:?}", a))
                .sum::<usize>()

            // println!("{:?}", line);
        }).sum::<usize>()
        // .collect();
}

fn part_2(input: &str) -> usize {
    let width: usize = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();

    let input: String = input
        .replace('\n', "")
        .chars()
        .map(|c| {
            if c.is_digit(10) || c == '*' {
                c
            } else {
                '.'
            }
        }).collect();

    let asterix_indices = input
        .char_indices()
        .filter_map(|(i, c)| if c == '*' { Some(i) } else { None })
        .collect::<Vec<_>>();

    let mut result = 0;

    for i in asterix_indices {
        let row = i / width;
        let col = i % width;
        let r1 = row.saturating_sub(1);
        let r2 = (row + 1).min(height);

        let mut num_count = 0;

        for r in r1..=r2 {
            let ri = r * width + col - 3;
            let mut box_line = input[ri..ri + 7].to_string();

            if r != row {
                box_line = box_line.replace('*', ".");
            } else {
                let lhs = &box_line[2..3];
                let rhs = &box_line[4..5];
                if lhs.parse::<u8>().is_ok() {
                    num_count += 1;
                }
    
                if rhs.parse::<u8>().is_ok() {
                    num_count += 1;
                }
            }

            num_count += &box_line[2..5]
                .split('.')
                .filter_map(|c| c.parse::<u32>().ok())
                .count();
        }

        let is_gear = num_count == 2;

        if is_gear {
            let gear: usize = (r1..=r2).into_iter().map(|r| {
                let ri = r * width + col - 3;
                let box_line = &input[ri..ri + 7];

                let short = box_line
                    .char_indices()
                    .filter(|(_, c)| c.is_digit(10))
                    .map(|(i, c)| (i, c.to_digit(10)))
                    .collect::<Vec<_>>();
                let groups = short
                    .group_by(|&a, &b| b.0 < a.0 + 2)        
                    .filter(|g| {
                        g.iter().map(|(i, _)| i).any(|&i| i >= 2 && i <= 4)
                    })
                    .map(|nums| nums.iter().map(|(_, n)| n.unwrap()).collect::<Vec<u32>>())
                    .map(|nums| {
                        nums.iter().rev().enumerate().map(move |(i, &n)| {
                            let m = 10usize.pow(i as u32);
                            (n as usize) * m
                        })
                        .sum::<usize>()
                    })
                    .filter(|&n| n > 0)
                    .product::<usize>();
                groups
            })
            .filter(|&n| n > 0)
            .product();

            result += gear;
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
        assert_eq!(part_1(TEST_INPUT), 4361);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 467835);
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