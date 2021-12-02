// https://adventofcode.com/2021/day/1

use std::io::{Cursor, BufRead};

const INPUT: &'static str = include_str!(r"..\data\day1.txt");

pub fn part1() -> i32 {
    let values = parse_input();

    count_increases(values)
}

pub fn part2() -> i32 {
    let values = parse_input().as_slice()
        .windows(3)
        .map(|w| -> i32 { w.iter().sum() })
        .collect();

    count_increases(values)
}

fn count_increases(data: Vec<i32>) -> i32 {
    data.as_slice()
        .windows(2)
        .fold(0, |count, win| {
            if win[1] > win[0] { count + 1 } else { count }
        })
}

fn parse_input() -> Vec<i32> {
    Cursor::new(INPUT)
        .lines()
        .map(|v| v.unwrap().parse::<i32>().unwrap())
        .collect()
}
