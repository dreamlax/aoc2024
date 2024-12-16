use std::fs::read;
use std::path::PathBuf;
use utils::timer::Timer;

fn count_xmas(board: &[u8]) -> usize {
    let board_width = 1 + board
        .iter()
        .position(|&ch| ch == b'\n')
        .expect("There should be a linefeed");
    let board_height = board.len() / board_width;

    assert_eq!(board.len() % board_width, 0, "Input is not rectangular!");

    macro_rules! is_mas {
        ($m: expr, $a: expr, $s: expr) => {
            $m == b'M' && $a == b'A' && $s == b'S'
        }
    }

    board
        .iter()
        .enumerate()
        .filter(|(_, &ch)| ch == b'X')
        .map(|(pos, _)| {
            let mut count = 0;
            let x = pos % board_width;
            let y = pos / board_width;

            let search_left = x > 2;
            let search_right = x < board_width - 3;
            let search_up = y > 2;
            let search_down = y < board_height - 3;

            // check left - simple shortcut since bytes are sequential
            if search_left && &board[pos-3..pos] == b"SAM" {
                count += 1;
            }

            // check right - follow same shortcut
            if search_right && &board[pos+1..pos+4] == b"MAS" {
                count += 1;
            }
            
            // check up
            if search_up && is_mas!(board[pos-board_width], board[pos-2*board_width], board[pos-3*board_width]) {
                count += 1;
            }

            // check down
            if search_down && is_mas!(board[pos+board_width], board[pos+2*board_width], board[pos+3*board_width]) {
                count += 1;
            }

            // check up/left
            if search_up && search_left && is_mas!(board[pos-board_width-1], board[pos-2*board_width-2], board[pos-3*board_width-3]) {
                count += 1;
            }

            // check down/left
            if search_down && search_left && is_mas!(board[pos+board_width-1], board[pos+2*board_width-2], board[pos+3*board_width-3]) {
                count += 1;
            }

            // check up/right
            if search_up && search_right && is_mas!(board[pos-board_width+1], board[pos-2*board_width+2], board[pos-3*board_width+3]) {
                count += 1;
            }

            // check down/right
            if search_down && search_right && is_mas!(board[pos+board_width+1], board[pos+2*board_width+2], board[pos+3*board_width+3]) {
                count += 1;
            }

            count
        })
        .sum()
}

fn count_x_mas(board: &[u8]) -> usize {
    let board_width = 1 + board
        .iter()
        .position(|&ch| ch == b'\n')
        .expect("There should be a linefeed");
    let board_height = board.len() / board_width;

    assert_eq!(board.len() % board_width, 0, "Input is not rectangular!");

    board
        .iter()
        .enumerate()
        .filter(|(pos, &ch)| {
            ch == b'A' && {
                let x = pos % board_width;
                let y = pos / board_width;
    
                if x == 0 || x == board_width - 1 || y == 0 || y == board_height - 1 {
                    return false;
                }
    
                let cross = [
                    board[pos-1-board_width],
                    board[pos+1+board_width],
                    board[pos-1+board_width],
                    board[pos+1-board_width]
                ];
    
                matches!(&cross, b"SMSM" | b"MSMS" | b"SMMS" | b"MSSM")
            }
        })
        .count()
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let input = read(path)
        .expect("Should be able to read from path");

    let answer = if !cfg!(feature = "part2") {
        count_xmas(&input)
    } else {
        count_x_mas(&input)
    };
    println!("Answer: {answer}");
}
