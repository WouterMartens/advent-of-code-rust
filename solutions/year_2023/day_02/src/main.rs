#![feature(test)]
#![allow(dead_code)]

extern crate test;

use utilities::read_input;

const PATH: &str = "input.txt";

const TEST_INPUT: &str =
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

fn main() {
    let input = read_input(PATH);
    // let input = TEST_INPUT;
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2(&input));
}

fn game_is_possible(id: usize, line: &str, max_values: &[u8; 3]) -> u8 {
    let line = line.split(": ").nth(1).unwrap().replace(';', ",").to_string();
    for cube in line.split(", ") {
        let cube = cube.split_whitespace().collect::<Vec<&str>>();
        match cube.as_slice() {
            [n, colour] => {
                let n = n.parse::<u8>().unwrap();

                match *colour {
                    "red" => if n > max_values[0] { return 0; },
                    "green" => if n > max_values[1] { return 0; },
                    "blue" => if n > max_values[2] { return 0; },
                    _ => panic!("Invalid colour"),
                }
            },
            _ => panic!("Invalid cube"),
        }
    }

    return id as u8;
}

fn min_cubes(line: &str) -> u32 {
    let mut max_values: [usize; 3] = [0; 3];
    let line = line.split(": ").nth(1).unwrap().replace(';', ",").to_string();
    for cube in line.split(", ") {
        let cube = cube.split_whitespace().collect::<Vec<&str>>();
        match cube.as_slice() {
            [n, colour] => {
                let n = n.parse::<usize>().unwrap();

                match *colour {
                    "red" => if n > max_values[0]  { max_values[0] = n },
                    "green" => if n > max_values[1] { max_values[1] = n; },
                    "blue" => if n > max_values[2] { max_values[2] = n; },
                    _ => panic!("Invalid colour"),
                }
            },
            _ => panic!("Invalid cube"),
        }
    }

    max_values.iter().fold(1, |acc, &x| acc * x) as u32
}

fn part_1(input: &str) -> u32 {
    const MAX_RGB: [u8; 3] = [12, 13, 14];
    let mut total = 0;
    for (i, line) in input.lines().enumerate() {
        let id = i + 1;
        total += game_is_possible(id, &line, &MAX_RGB) as u32;
    }
    total
}

fn part_2(input: &str) -> u32 {
    input.lines().map(min_cubes).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(game_is_possible(1, "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", &[12, 13, 14]), 1);
        assert_eq!(game_is_possible(2, "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", &[12, 13, 14]), 2);
        assert_eq!(game_is_possible(5, "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", &[12, 13, 14]), 5);
        assert_eq!(game_is_possible(3, "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", &[12, 13, 14]), 0);
        assert_eq!(game_is_possible(4, "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", &[12, 13, 14]), 0);
        assert_eq!(part_1(TEST_INPUT), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(min_cubes("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 48);
        assert_eq!(min_cubes("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 12);
        assert_eq!(min_cubes("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 1560);
        assert_eq!(min_cubes("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 630);
        assert_eq!(min_cubes("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 36);
        assert_eq!(part_2(TEST_INPUT), 2286);
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