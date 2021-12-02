// https://adventofcode.com/2021/day/2

use std::io::{Cursor, BufRead};

const INPUT: &'static str = include_str!(r"..\data\day2.txt");

pub fn part1() -> i32 {
    let mut depth = 0;
    let mut position = 0;

    let commands = parse_input();
    for (action, value) in commands {
        match action.as_str() {
            "up" => depth -= value,
            "down" => depth += value,
            "forward" =>  position += value,
            _ => println!("unknown command"),
        }
    }

    depth * position
}

pub fn part2() -> i32 {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    let commands = parse_input();
    for (action, value) in commands {
        match action.as_str() {
            "up" => aim -= value,
            "down" => aim += value,
            "forward" => {
                position += value;
                depth += aim * value;
            },
            _ => println!("unknown command"),
        }
    }

    depth * position
}

fn parse_input() -> Vec<(String, i32)> {
    Cursor::new(INPUT)
        .lines()
        .map(|s| -> (String, i32) {
            let line = s.unwrap();
            let mut parts = line.split_whitespace();

            let action = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().parse::<i32>().unwrap();
            
            (action, value)
        }).collect()
}
