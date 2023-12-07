#![feature(test)]
#![allow(dead_code)]

extern crate test;
use std::collections::HashSet;

use utilities::read_input;

const PATH: &str = "input.txt";

const TEST_INPUT_ORIGINAL: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

const TEST_INPUT_CUSTOM: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
AAAAA 25
1111T 35
12345 60";

const TEST_INPUT_CUSTOM_2: &str = "32T3K 765
T323K 777
T55J5 684
555TJ 690
KK677 28
677KK 29
KTJJT 220
JJTTK 221
QQQJA 483
JAQQQ 482
AAAAA 25
QQQQQ 36
1111T 35
T1111 39
12345 60
K5432 73";

const TEST_INPUT_CUSTOM_3: &str = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Default)]
struct Hand {
    cards: String,
    original: String,
    bid: u16,
    value: usize,
}

fn main() {
    let input = read_input(PATH);
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2(&input));
}

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let original = parts.next()?.to_string();
            let cards = original
                .replace('T', "a")
                .replace('J', "b")
                .replace('Q', "c")
                .replace('K', "d")
                .replace('A', "e");

            let bid = parts.last()?.parse::<u16>().ok()?;
            let value = usize::from_str_radix(&cards, 15).expect("Should be able to parse");

            Some(Hand {
                cards,
                bid,
                value,
                original,
            })
        })
        .collect::<Vec<Hand>>()
}

fn categorize_hands(hands: &[Hand]) -> Vec<(HandType, &Hand)> {
    hands
        .iter()
        .map(|hand| {
            let mut unique = HashSet::new();
            hand.cards.chars().for_each(|c| _ = unique.insert(c));

            let max_unique_values = unique
                .iter()
                .map(|unique_c| hand.cards.chars().filter(|c| c == unique_c).count())
                .max();

            let hand_type = match unique.len() {
                1 => HandType::FiveOfAKind,
                2 => match max_unique_values {
                    Some(4) => HandType::FourOfAKind,
                    Some(3) => HandType::FullHouse,
                    _ => panic!("Invalid max unique value in hand for unique length arm 2"),
                },
                3 => match max_unique_values {
                    Some(2) => HandType::TwoPair,
                    Some(3) => HandType::ThreeOfAKind,
                    _ => panic!("Invalid max unique value in hand for unique length arm 3"),
                },
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                _ => panic!("Invalid hand"),
            };

            (hand_type, hand)
        })
        .collect::<Vec<_>>()
}

fn part_1(input: &str) -> usize {
    let hands = parse(input);
    let mut hands = categorize_hands(&hands);

    hands.sort_by(|(a_type, a_hand), (b_type, b_hand)| {
        a_type
            .cmp(&b_type)
            .then_with(|| a_hand.value.cmp(&b_hand.value))
    });

    hands
        .iter()
        .enumerate()
        .map(|(i, &(_, hand))| (i + 1) * hand.bid as usize)
        .sum::<usize>()
}

fn part_2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT_ORIGINAL), 6440);
        assert_eq!(part_1(TEST_INPUT_CUSTOM), 9125);
        assert_eq!(part_1(TEST_INPUT_CUSTOM_2), 34933);
        assert_eq!(part_1(TEST_INPUT_CUSTOM_3), 6592);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(part_1(TEST_INPUT_ORIGINAL), 5905);
        assert_eq!(part_2(TEST_INPUT_CUSTOM_3), 6839);
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
