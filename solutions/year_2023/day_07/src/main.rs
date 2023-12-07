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

const TEST_INPUT_CUSTOM: &str = "2345A 1
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

#[derive(Debug, Default, Clone)]
struct Hand {
    cards: String,
    bid: u16,
    value: usize,
}

fn main() {
    let input = read_input(PATH);
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2(&input));
}

fn parse(input: &str, replace_joker: bool) -> Vec<Hand> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let original = parts.next()?;
            let cards = original
                .chars()
                .map(|c| match c {
                    'T' => 'a',
                    'J' => 'b',
                    'Q' => 'c',
                    'K' => 'd',
                    'A' => 'e',
                    _ => c,
                })
                .collect::<String>();

            let bid = parts.last()?.parse::<u16>().ok()?;

            let value = match replace_joker {
                false => usize::from_str_radix(&cards, 15).expect("Should be able to parse"),
                true => usize::from_str_radix(&cards.replace('b', "1"), 15)
                    .expect("Should be able to parse"),
            };

            Some(Hand { cards, bid, value })
        })
        .collect::<Vec<Hand>>()
}

fn categorize_hands(hands: &mut [Hand], replace_joker: bool) -> Vec<(HandType, &mut Hand)> {
    hands
        .iter_mut()
        .map(|hand| {
            let mut unique = hand.cards.chars().collect();
            let (max_char, mut max_unique_values) =
                find_max_unique(&hand.cards, &unique, replace_joker);

            if replace_joker && hand.cards.contains('b') {
                hand.cards = hand.cards.replace('b', &max_char.to_string());
                unique = hand.cards.chars().collect::<HashSet<_>>();
                max_unique_values = find_max_unique_values(&hand.cards, &unique);
            }

            let unique_count = unique.len();
            let hand_type = determine_hand_type(unique_count, max_unique_values);

            (hand_type, hand)
        })
        .collect()
}

fn find_max_unique(cards: &str, unique: &HashSet<char>, replace_joker: bool) -> (char, usize) {
    let mut max_char = ' ';
    let mut max_unique_values = 0;

    for &unique_c in unique {
        let count = cards
            .chars()
            .filter(|&c| c == unique_c && (c != 'b' || !replace_joker))
            .count();
        if count > max_unique_values {
            max_unique_values = count;
            max_char = unique_c;
        }
    }

    (max_char, max_unique_values)
}

fn find_max_unique_values(cards: &str, unique: &HashSet<char>) -> usize {
    unique
        .iter()
        .map(|&c| cards.chars().filter(|&x| x == c).count())
        .max()
        .unwrap_or(0)
}

fn determine_hand_type(unique_count: usize, max_unique_values: usize) -> HandType {
    match unique_count {
        1 => HandType::FiveOfAKind,
        2 => match max_unique_values {
            3 => HandType::FullHouse,
            4 => HandType::FourOfAKind,
            v => panic!("Invalid value {} for unique length arm 2", v),
        },
        3 => match max_unique_values {
            2 => HandType::TwoPair,
            3 => HandType::ThreeOfAKind,
            v => panic!("Invalid value {} for unique length arm 3", v),
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        h => panic!("Invalid hand {}", h),
    }
}

fn sort_hands(hands: &mut [(HandType, &mut Hand)]) {
    hands.sort_by(|(a_type, a_hand), (b_type, b_hand)| {
        a_type
            .cmp(&b_type)
            .then_with(|| a_hand.value.cmp(&b_hand.value))
    });
}

fn calculate_score(enumerated_hands: &Vec<(HandType, &mut Hand)>) -> usize {
    enumerated_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &(_, ref hand))| {
            acc + (i + 1) * hand.bid as usize
        })
}

fn solve(input: &str, replace_joker: bool) -> usize {
    let mut hands = parse(input, replace_joker);
    let mut hands = categorize_hands(&mut hands, replace_joker);
    sort_hands(&mut hands);
    calculate_score(&hands)
}

fn part_1(input: &str) -> usize {
    let replace_joker = false;
    solve(input, replace_joker)
}

fn part_2(input: &str) -> usize {
    let replace_joker = true;
    solve(input, replace_joker)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT_ORIGINAL), 6440);
        assert_eq!(part_1(TEST_INPUT_CUSTOM), 6592);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT_ORIGINAL), 5905);
        assert_eq!(part_2(TEST_INPUT_CUSTOM), 6839);
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
