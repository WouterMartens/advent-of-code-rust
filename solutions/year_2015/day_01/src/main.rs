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

fn part_1(input: &str) -> i64 {
    part_1_count_bytes(&input)
}

fn part_2(input: &str) -> i64 {
    part_2_loop_bytes(&input)
}

fn part_1_loop_chars(input: &str) -> i64 {
    let mut floor = 0;
    const UP: char = '(';

    for c in input.chars() {
        if c == UP {
            floor += 1;
        } else {
            floor -= 1;
        }
    }

    floor
}

fn part_1_loop_bytes(input: &str) -> i64 {
    let mut floor = 0;
    const UP: u8 = b'(';

    for b in input.bytes() {
        // Faster than match b
        if b == UP {
            floor += 1;
        } else {
            floor -= 1;
        }
    }

    floor
}

fn part_1_count_bytes(input: &str) -> i64 {
    let length = input.bytes().count() as i64;
    let up_count = input.bytes().filter(|&b| b == b'(').count() as i64;
    let floor = up_count - (length - up_count);

    floor
}

fn part_2_loop_bytes(input: &str) -> i64 {
    let mut floor = 0;
    let mut result = 0;
    
    const UP: u8 = b'(';

    for (i, b) in input.bytes().enumerate() {
        if b == UP {
            floor += 1;
        } else {
            floor -= 1;
        }
        
        if floor == -1 {
            result = i;
            break;
        }
    }

    result as i64 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("(())"), 0);
        assert_eq!(part_1("()()"), 0);
        assert_eq!(part_1("((("), 3);
        assert_eq!(part_1("(()(()("), 3);
        assert_eq!(part_1("))((((("), 3);
        assert_eq!(part_1("())"), -1);
        assert_eq!(part_1("))("), -1);
        assert_eq!(part_1(")))"), -3);
        assert_eq!(part_1(")())())"), -3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(")"), 1);
        assert_eq!(part_2("()())"), 5);
    }

    #[bench]
    fn bench_part_1_loop_chars(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1_loop_chars(&input));
    }

    #[bench]
    fn bench_part_1_loop_bytes(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1_loop_bytes(&input));
    }

    #[bench]
    fn bench_part_1_count_bytes(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1_count_bytes(&input));
    }

    #[bench]
    fn bench_part_2_loop_bytes(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_loop_bytes(&input));
    }
}