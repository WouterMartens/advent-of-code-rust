#![feature(test)]
#![allow(dead_code)]

extern crate test;
use itertools::Itertools;
use utilities::read_input;

const PATH: &str = "input.txt";
const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

fn main() {
    let input = read_input(PATH);
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2(&input, 1_000_000));
}

fn part_1(input: &str) -> isize {
    let mut field: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let line_length = input.lines().last().unwrap().chars().count();

    let mut i = 0;
    let mut field_length = field.iter().len();

    while i < field_length {
        let row = &field[i];
        if !row.iter().any(|&c| c != '.') {
            field.insert(i, vec!['.'; line_length]);
            i += 2;
            field_length += 1;
        } else {
            i += 1;
        }
    }

    let mut i = 0; // row
    let mut j = 0; // column
    let mut line_length = line_length;

    while j < line_length {
        let mut all_dots = true;
        i = 0;

        while i < field_length {
            if field[i][j] != '.' {
                all_dots = false;
                break;
            }

            i += 1;
        }

        if all_dots {
            for row in &mut field {
                row.insert(j, '.')
            }

            line_length += 1;
            j += 2;
        } else {
            j += 1;
        }
    }

    for line in &field {
        println!(
            "{:?}",
            line.iter().map(|c| c.to_string()).collect::<String>()
        );
    }
    println!("{} {}", field.len(), field[0].len());

    field
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|&(_, &char)| char == '#')
                .map(move |(column, _)| (row, column))
        })
        .combinations(2)
        .map(|coords| {
            let ((x1, y1), (x2, y2)) = (coords[0], coords[1]);
            (x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()
        })
        .inspect(|a| println!("{a:?}"))
        .sum::<isize>()
}

fn part_2(input: &str, expansion: usize) -> isize {
    let field: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let line_length = field[0].len();

    // Finding # coordinates
    let coordinates: Vec<(usize, usize)> = field
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, &c)| {
                if c == '#' {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            })
        })
        .collect();

    // Finding empty rows and columns indices
    let empty_rows: Vec<usize> = field
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(index, _)| index)
        .collect();

    let empty_columns: Vec<usize> = (0..line_length)
        .filter(|&col| field.iter().all(|row| row[col] == '.'))
        .collect();

    let mut total_distance = 0;
    for coordinate in coordinates.iter().combinations(2) {
        let ((x1, y1), (x2, y2)) = (coordinate[0], coordinate[1]);

        let row_count = empty_rows
            .iter()
            .filter(|&row| row > x1.min(x2) && row < x1.max(x2))
            .count() as isize;

        let column_count = empty_columns
            .iter()
            .filter(|&col| col > y1.min(y2) && col < y1.max(y2))
            .count() as isize;

        let x_delta = (*x1 as isize - *x2 as isize).abs();
        let y_delta = (*y1 as isize - *y2 as isize).abs();
        let expansion_value = (row_count + column_count) * (expansion as isize - 1); // * expansion as isize / 2 as isize;
        let distance = x_delta + y_delta + expansion_value;

        total_distance += distance;
    }

    total_distance
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 374);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT, 2), 374);
        assert_eq!(part_2(TEST_INPUT, 10), 1030);
        assert_eq!(part_2(TEST_INPUT, 100), 8410);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2(&input, 1_000_000));
    }
}
