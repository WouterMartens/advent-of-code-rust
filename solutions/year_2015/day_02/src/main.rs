#![feature(test)]
#![allow(dead_code)]

extern crate test;
use utilities::read_input;

const PATH: &str = "input.txt";

struct Present {
    dimensions: [u32; 3],
    lw: u32,
    wh: u32,
    hl: u32,
}

impl Present {
    fn new(dimensions: [u32; 3]) -> Self {
        let lw: u32 = dimensions[0] * dimensions[1];
        let wh: u32 = dimensions[1] * dimensions[2];
        let hl: u32 = dimensions[2] * dimensions[0];

        Present {
            dimensions,
            lw,
            wh,
            hl,
        }
    }

    fn surface_area(&self) -> u32 {
        2 * (self.lw + self.wh + self.hl)
    }

    fn smallest_side(&self) -> u32 {
        self.lw.min(self.wh).min(self.hl)
    }

    fn ribbon_fold(&self) -> u32 {
        let smallest_sides = self.dimensions.iter().fold((u32::MAX, u32::MAX), |acc, &x| {
            let (min1, min2) = acc;
            if x < min1 {
                (x, min1)
            } else if x < min2 {
                (min1, x)
            } else {
                (min1, min2)
            }
        });

        let (min1, min2) = smallest_sides;
        let bow: u32 = self.dimensions.iter().product();
        let wrap = 2 * (min1 + min2);

        bow + wrap
    }

    fn ribbon_loop(&self) -> u32 {
        let (mut min1, mut min2) = (u32::MAX, u32::MAX);
        let mut bow = 1;

        for &dimension in &self.dimensions {
            if dimension <= min1 {
                min2 = min1;
                min1 = dimension;
            } else if dimension <= min2 {
                min2 = dimension;
            }

            bow *= dimension;
        }

        let wrap = 2 * (min1 + min2);
        wrap + bow
    }

    fn ribbon(&self) -> u32 {
        let mut sorted_dimensions = self.dimensions;
        sorted_dimensions.sort();

        // let bow: u32 = self.dimensions.iter().fold(1, |acc, &e| acc * e);
        let bow: u32 = self.dimensions.iter().product();
        let wrap: u32 = sorted_dimensions[0] * 2 + sorted_dimensions[1] * 2; 
        
        bow + wrap
    }
}

fn main() {
    let input = read_input(PATH);
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    part_1_struct(&input)
}

fn part_2(input: &str) -> u32 {
    part_2_struct_fold(&input)
}

fn part_1_vector(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
           let line: Vec<u32> = line
                .split('x')
                .map(|x| x.parse().unwrap())
                .collect();

            let l: u32 = line[0]; // length
            let w: u32 = line[1]; // width
            let h: u32 = line[2]; // height

            let top = l*w;
            let left = w*h;
            let front = h*l;

            // let smallest = [top, left, front].into_iter().min().unwrap();
            let extra: u32 = top.min(left).min(front);

            2*top + 2*left + 2*front + extra
        })
        // .reduce(|acc, e| acc + e).unwrap()
        .sum()
}

fn part_1_array(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut dims: [u32; 3] = [0; 3];

            for (i, value) in line.split('x').enumerate() {
                dims[i] = value.parse().unwrap();
            }

            let top: u32 = dims[0] * dims[1];
            let left: u32 = dims[1] * dims[2];
            let front: u32 = dims[2] * dims[0];

            // let smallest: u32 = [top, left, front].into_iter().min().unwrap();
            let extra: u32 = top.min(left).min(front);

            2*top + 2*left + 2*front + extra
        })
        .sum()
}

fn part_1_struct(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut dimensions: [u32; 3] = [0; 3];

            for (i, value) in line.split('x').enumerate() {
                dimensions[i] = value.parse().unwrap();
            }

            let box_dims = Present::new(dimensions);

            box_dims.surface_area() + box_dims.smallest_side()
        })
        .sum()
}

fn part_2_struct(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut dimensions: [u32; 3] = [0; 3];

            for (i, value) in line.split('x').enumerate() {
                dimensions[i] = value.parse().unwrap();
            }

            let box_dims = Present::new(dimensions);
            box_dims.ribbon()
        })
        .sum()
}

fn part_2_struct_fold(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut dimensions: [u32; 3] = [0; 3];

            for (i, value) in line.split('x').enumerate() {
                dimensions[i] = value.parse().unwrap();
            }

            let box_dims = Present::new(dimensions);
            box_dims.ribbon_fold()
        })
        .sum()
}

// Unit tests! All the examples given in the puzzle descriptions are added here as unit tests.
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("2x3x4"), 58);
        assert_eq!(part_1("1x1x10"), 43);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("2x3x4"), 34);
        assert_eq!(part_2("1x1x10"), 14);
    }

    #[bench]
    fn bench_part_1_vector(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1_vector(&input));
    }

    #[bench]
    fn bench_part_1_array(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1_array(&input));
    }

    #[bench]
    fn bench_part_1_struct(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_1_struct(&input));
    }

    #[bench]
    fn bench_part_2_struct_fold(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_struct_fold(&input));
    }

    #[bench]
    fn bench_part_2_struct(b: &mut Bencher) {
        let input = read_input(PATH);
        b.iter(|| part_2_struct(&input));
    }
}