#![feature(test)]
#![allow(dead_code)]

extern crate test;
use utilities::read_input;
use std::collections::HashSet;

const PATH: &str = "input.txt";
const TEST_INPUT: &str = 
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

fn main() {
    let input = read_input(PATH);
    // let input = TEST_INPUT;
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            line.split(": ")
                .nth(1)
                .map(|part| {
                    part.split(" | ")
                        .map(|s| s
                            .split_ascii_whitespace()
                            .map(|n| n
                                .parse::<u8>()
                                .expect("Should all be numbers"))
                            .collect::<Vec<_>>())
                        .collect::<Vec<_>>()
                })
        })
        .map(|nums| {
            let wins: HashSet<&u8> = nums[0].iter().collect();
            let me: HashSet<&u8> = nums[1].iter().collect();

            let matching: HashSet<_> = wins.intersection(&me).collect();
            if matching.is_empty() {
                0
            } else {
                1 << matching.len().saturating_sub(1) as u32
            }
        }).sum()
}

fn part_2(input: &str) -> u32 {
    const TOTAL_CARDS: usize = 201;
    let mut cards: [u32; TOTAL_CARDS] = [0; TOTAL_CARDS];

    input
        .lines()
        .filter_map(|line| {
            line.split(": ")
                .nth(1)
                .map(|part| {
                    part.split(" | ")
                        .map(|s| s
                            .split_ascii_whitespace()
                            .map(|n| n
                                .parse::<u8>()
                                .expect("Should all be numbers"))
                            .collect::<HashSet<_>>())
                        .collect::<Vec<_>>()
                })
        })
        .map(|nums| {
            let wins: HashSet<u8> = nums[0].iter().copied().collect();
            let me: HashSet<u8> = nums[1].iter().copied().collect();
            let matches = wins.intersection(&me).copied().collect::<Vec<_>>();
            matches.len() 
        })
        .enumerate()
        .for_each(|(card, match_num)| {
            let length = if card + match_num < TOTAL_CARDS {
                card + match_num
            } else {
                TOTAL_CARDS
            };

            cards[card] += 1;

            (card + 1..=length).for_each(|i| {
                cards[i] += cards[card] as u32;
            });
        });
        
    cards.iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 8);
        assert_eq!(part_1("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), 2);
        assert_eq!(part_1("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), 2);
        assert_eq!(part_1("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), 1);
        assert_eq!(part_1("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), 0);
        assert_eq!(part_1("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 0);
        assert_eq!(part_1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 30);
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