#![feature(test)]
#![allow(dead_code)]

extern crate test;

use utilities::read_input;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use indicatif::ProgressBar;

const PATH: &str = "input.txt";
const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

struct Range {
    source: i64,
    delta: i64,
    length: i64,
}

struct Map {
    ranges: Vec<Range>,
}

struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn new(input: &str) -> Self {
        let mut seeds: Vec<i64> = Vec::new();
        let mut maps: Vec<Map> = Vec::new();
        let mut seen_first_colon = false;
        let mut lines = input.lines();
    
        while let Some(line) = lines.next() {
            if let Some(_) = line.find(':') {
                if !seen_first_colon {
                    seeds = line
                            .split_once(": ")
                            .unwrap().1
                            .split_ascii_whitespace()
                            .filter_map(|s| s.parse::<i64>().ok())
                            .collect::<Vec<_>>();
                    seen_first_colon = true;
                } else {
                    process_map_lines(&mut lines, &mut maps);
                }
            } 
        }
    
        Self { seeds, maps }
    }

    fn traverse_seed(&self, seed: i64) -> i64 {
        let mut value = seed;
        
        for map in &self.maps {
            for range in &map.ranges {
                let in_range = value >= range.source && value <= range.source + range.length;
                if in_range {
                    value += range.delta;
                    break;
                }
            }
        }

        value
    }

    fn traverse(&self) -> i64 {
        self.seeds
            .iter()
            .map(|seed| self.traverse_seed(*seed))
            .min()
            .expect("Should have a result value")
    }

    fn traverse_range(&self) -> i64 {
        let total_iterations: usize = self.seeds.chunks(2).map(|chunk| chunk[1] as usize).sum();
        let progress = ProgressBar::new(total_iterations as u64);

        let result = self
            .seeds
            .chunks(2)
            .par_bridge()
            .flat_map(|chunk| {
                chunk[0]..(chunk[0] + chunk[1])
            })
            .map(|seed| {
                progress.inc(1);
                self.traverse_seed(seed)
            })
            .min()
            .expect("Should have a minumum result value");

        progress.finish();
        result
    }
}

fn main() {
    let input = read_input(PATH);
    // let input = TEST_INPUT;
    // println!("Part 1 answer: {:?}", part_1(&input));
    println!("Part 2 answer: {:?}", part_2(&input));
}

fn process_map_lines(lines: &mut dyn Iterator<Item = &str>, maps: &mut Vec<Map>) {
    let mut ranges: Vec<Range> = Vec::new();

    while let Some(next_line) = lines.next() {
        if next_line.is_empty() {
            break;
        }

        let mut values = next_line
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<i64>().ok());

        if let (Some(destination), Some(source), Some(length)) = 
            (values.next(), values.next(), values.next())
        {
            ranges.push( Range {
                source,
                length,
                delta: destination - source,
            });
        }
    }

    maps.push(Map { ranges } );
}

fn part_1(input: &str) -> i64 {
    let almanac = Almanac::new(&input);
    almanac.traverse()
}

fn part_2(input: &str) -> i64 {
    let almanac = Almanac::new(&input);
    almanac.traverse_range()
}