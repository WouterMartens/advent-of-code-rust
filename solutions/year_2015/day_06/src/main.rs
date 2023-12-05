#![feature(test)]
#![allow(dead_code)]

extern crate test;
use std::fmt;
use utilities::read_input;

const PATH: &str = "input.txt";
const SIZE: usize = 1000;

fn main() {
    let input = read_input(PATH);
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    part_1_v1(&input)
}

fn part_2(input: &str) -> u32 {
    part_2_nested(&input)
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(xy: &str) -> Self {
        let mut parts = xy.split(',');
        let x = Self::parse(parts.next().unwrap());
        let y = Self::parse(parts.next().unwrap());

        Point { x, y }
    }

    fn parse(coordinate: &str) -> usize {
        coordinate.parse::<usize>().expect("Input should always be x,y as a string")
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq)]
enum Command {
    On,
    Off,
    Toggle,
}

struct Grid {
    lights: [[u32; SIZE]; SIZE],
}

impl Grid {
    fn new() -> Self {
        Grid { lights: [[0; SIZE]; SIZE] }
    }

    fn apply_command(&mut self, command: &Command, top_left: Point, bottom_right: Point) {
        // for (row, col) in itertools::iproduct!(top_left.y..=bottom_right.y, top_left.x..=bottom_right.x) {
        // ^Slightly slower
        for col in top_left.x..=bottom_right.x {
            for row in top_left.y..=bottom_right.y {
                let current_light = &mut self.lights[row][col];
                match command {
                    Command::On => {
                        *current_light += 1;
                    }
                    Command::Off => {
                        if *current_light > 0 {
                            *current_light -= 1;
                        }
                    },
                    Command::Toggle => {
                        *current_light += 2;
                    },
                    // Unnecessary since we're using an enum
                    // _ => panic!("Unexpected command"), 
                }
            }
        }
    }

    fn lights_on(&self) -> u32 {
        self.lights.iter().flatten().sum()
    }
}

fn part_1_v1(input: &str) -> u32 {
    let lines = input.lines().take(SIZE * SIZE);
    let mut grid = [[0; SIZE]; SIZE];
    // let mut lights_on = 0;
    
    for line in lines {
        let i: Vec<&str> =  line.split_whitespace().collect();
        let (switch, top_left, bottom_right) = match i[0] {
            "turn" => (i[1], Point::new(i[2]), Point::new(i[4])),
            "toggle" => (i[0], Point::new(i[1]), Point::new(i[3])),
            _ => panic!("Invalid instruction"),
        };

        let (x1, y1) = (top_left.x, top_left.y);
        let (x2, y2) = (bottom_right.x, bottom_right.y);

        for col in x1..=x2 {
            for row in y1..=y2 {
                grid[row][col] = match switch {
                    "on" => 1,
                    "off" => 0,
                    "toggle" => if grid[row][col] == 1 { 0 } else { 1 },
                    _ => grid[row][col],
                };

                // Possibly use bit operations instead (would save the use of if statement)
                // "on" => |val: u64, bit: u64| val | bit,
                // "off" => |val: u64, bit: u64| val & !bit,
                // "toggle" => |val: u64, bit: u64| val ^ bit,
                // _ => return,
            }
        }
    }
    // Possibly keep a counter in the loop instead
    // ^Is very slow
    grid.iter().flatten().sum()
}

fn part_2_v1(input: &str) -> u32 {
    let lines = input.lines();
    let mut grid = [[0; SIZE]; SIZE];
    
    for line in lines {
        let i: Vec<&str> =  line.split_whitespace().collect();
        let (switch, top_left, bottom_right) = match i[0] {
            "turn" => (i[1], Point::new(i[2]), Point::new(i[4])),
            "toggle" => (i[0], Point::new(i[1]), Point::new(i[3])),
            _ => panic!("Invalid instruction"),
        };

        let (x1, y1) = (top_left.x, top_left.y);
        let (x2, y2) = (bottom_right.x, bottom_right.y);

        for col in x1..=x2 {
            for row in y1..=y2 {
                grid[row][col] = match switch {
                    "on" => grid[row][col] + 1,
                    "off" => if grid[row][col] > 0 { grid[row][col] - 1 } else { 0 },
                    "toggle" => grid[row][col] + 2,
                    _ => grid[row][col],
                };
                // dbg!(format!("{col} {row} {}", grid[row][col]));
            }
        }
    }
    grid.iter().flatten().sum()
}

// Also tried a "flat" approach resulting in similar speed
fn part_2_nested(input: &str) -> u32 {
    let mut grid = Grid::new();
    
    for line in input.lines().take(SIZE * SIZE) {
        let mut parts = line.split_whitespace();

        let command = match parts.next() {
            Some("turn") => {
                match parts.next() {
                    Some("on") => Command::On,
                    Some("off") => Command::Off,
                    _ => panic!("Invalid command"),
                }
            },
            Some("toggle") => {
                Command::Toggle
            },
            _ => panic!("Invalid instruction"),
        };

        let top_left = Point::new(parts.next().unwrap());
        let bottom_right = Point::new(parts.skip(1).next().unwrap());

        grid.apply_command(&command, top_left, bottom_right)
    }
    grid.lights_on()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    #[ignore]
    fn test_part_1() {
        // assert_eq!(SIZE, 3);
        assert_eq!(part_1("turn on 0,0 through 2,2"), 9);
        assert_eq!(part_1("turn on 0,0 through 2,2\ntoggle 0,0 through 2,0"), 6);
        assert_eq!(part_1("turn on 0,0 through 2,2\ntoggle 0,0 through 2,0\nturn off 1,1 through 1,1"), 5);
    }

    // 
    #[test]
    fn test_part_2() {
        // assert_eq!(SIZE, 3);
        assert_eq!(part_2("turn on 0,0 through 2,2"), 9);
        assert_eq!(part_2("turn on 0,0 through 2,2\ntoggle 0,0 through 2,0"), 15);
        assert_eq!(part_2("turn on 0,0 through 2,2\ntoggle 0,0 through 2,0\nturn off 1,1 through 1,1"), 14);
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
        b.iter(|| part_2_v1(&input));
    }

    #[bench]
    fn bench_part_2_nested(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_nested(&input));
    }
}