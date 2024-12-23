use utils::timer::Timer;
use std::fs::read;
use std::path::PathBuf;

mod board;

use board::{Board,Cell};

fn main() {
    let _timer = Timer::new();
    
    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let input = read(path)
        .expect("Should be able to read from input");

    let split_point = input
        .windows(2)
        .position(|ch| ch[0] == b'\n' && ch[1] == b'\n')
        .expect("Should have double line break in input");

    let mut board: Board = input[..split_point].into();
    let instructions_data = &input[split_point+1..];

    for i in instructions_data {
        board.process_instruction(*i);
    }

    let answer: usize = board.cells.iter()
        .enumerate()
        .filter(|(_idx, cell)| **cell == Cell::Box)
        .map(|(idx, _cell)| idx / board.width * 100 + idx % board.width)
        .sum();
    
    println!("Answer: {answer}");
}
