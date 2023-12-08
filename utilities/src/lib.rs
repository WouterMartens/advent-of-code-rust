use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("Input file path should exist")
}

pub fn isqrt(value: isize) -> isize {
    if value < 2 {
        return value;
    }

    let mut op = value;
    let mut res = 0;
    let mut one = 1 << (value.ilog2() & !1);

    while one != 0 {
        if op >= res + one {
            op -= res + one;
            res = (res >> 1) + one;
        } else {
            res >>= 1;
        }
        one >>= 2;
    }

    res
}
