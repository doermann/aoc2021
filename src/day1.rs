// https://adventofcode.com/2021/day/1

use std::io::{Cursor, BufRead};

const INPUT: &'static str = include_str!(r"..\data\day1.txt");

pub fn part1() -> i32 {
    let values = parse_i32_lines(INPUT);

    count_increases(values)
}

pub fn part2() -> i32 {
    let values = parse_i32_lines(INPUT).as_slice()
        .windows(3)
        .map(|w| -> i32 { w.iter().sum() })
        .collect::<Vec<i32>>();

    count_increases(values)
}

/// counts the number of times the sequence increases
fn count_increases(data: Vec<i32>) -> i32 {
    data.as_slice()
        .windows(2)
        .fold(0, |count, win| {
            if win[1] > win[0] { count + 1 } else { count }
        })
}

/// parse 1 number per line. !!panics on bad data!!
fn parse_i32_lines(text: &str) -> Vec<i32> {
    Cursor::new(text)
        .lines()
        .map(|v| v.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
