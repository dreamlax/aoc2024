#[cfg(feature="part2")]
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use utils::timer::Timer;


#[cfg(not(feature = "part2"))]
fn get_answer(input: &str) -> i32 {
    let (mut left, mut right) = input
        .lines()
        .map(|l| l
            .split_once(|c: char| c.is_whitespace())
            .expect("Lines should be separated by whitespace"))
        .map(|(left, right)| (
            left
                .parse::<i32>()
                .unwrap(),
            right
                .trim()
                .parse::<i32>()
                .unwrap()))
        .collect::<(Vec<i32>, Vec<i32>)>();

    left.sort();
    right.sort();

    left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[cfg(feature = "part2")]
fn get_answer(input: &str) -> i32 {
    let (left, right) = input
        .lines()
        .map(|l| l
            .split_once(|c: char| c.is_whitespace())
            .expect("Lines should be separated by whitespace"))
        .fold((Vec::<i32>::new(), HashMap::<i32, i32>::new()), |(mut left_list, mut right_list), (left, right)| {
            left_list.push(left.parse().expect("Left column should be parseable"));
            right_list
                .entry(right.trim().parse().expect("Right column should be parseable"))
                .and_modify(|curr| *curr += 1)
                .or_insert(1);

            (left_list, right_list)
        });

    left
        .iter()
        .map(|v| v * right.get(&v).unwrap_or(&0))
        .sum()
}

fn main() {
    let _t = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let input = read_to_string(path)
        .expect("Should be able to read from path");

    let answer = get_answer(&input);
    println!("Answer: {answer}");
}
