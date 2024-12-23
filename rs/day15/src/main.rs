use utils::timer::Timer;
use std::fs::read;
use std::path::PathBuf;

mod board;
mod part1board;
mod part2board;

use board::Board;
use part1board::Part1Board;
use part2board::Part2Board;

fn get_answer<'a, T: Board<'a>>(input: &'a [u8]) -> usize {
    let split_point = input
        .windows(2)
        .position(|ch| ch[0] == b'\n' && ch[1] == b'\n')
        .expect("Should have double line break in input");

    let mut board: T = input[..split_point].into();
    let instructions_data = &input[split_point+1..];

    for i in instructions_data {
        board.process_instruction(*i);
    }

    board.sum_gps()
}

fn main() {
    let _timer = Timer::new();
    
    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let input = read(path)
        .expect("Should be able to read from input");

    let answer = if cfg!(feature = "part2") {
        get_answer::<Part2Board>(&input)
    }
    else {
        get_answer::<Part1Board>(&input)
    };
    
    println!("Answer: {answer}");
}
