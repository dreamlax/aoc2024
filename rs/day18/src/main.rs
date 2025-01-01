use utils::timer::Timer;
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::path::PathBuf;

const BOARD_WIDTH: usize = 71;
const BOARD_HEIGHT: usize = 71;

fn find_shortest_path(blocks: &HashSet<usize>) -> usize {
    let mut queue = VecDeque::new();
    let mut board = [usize::MAX; BOARD_WIDTH * BOARD_HEIGHT];

    queue.push_back((0usize, 0usize));
    while let Some((pos, score)) = queue.pop_front() {
        if blocks.contains(&pos) {
            continue;
        }

        if score >= board[pos] {
            continue;
        }

        board[pos] = score;

        if pos % BOARD_WIDTH != 0 {
            // go left
            queue.push_back((pos - 1, score + 1));
        }
        if pos % BOARD_WIDTH != BOARD_WIDTH - 1 {
            // go right
            queue.push_back((pos + 1, score + 1));
        }
        if pos > BOARD_WIDTH {
            // go up
            queue.push_back((pos - BOARD_WIDTH, score + 1));
        }
        if pos + BOARD_WIDTH < board.len() {
            // go down
            queue.push_back((pos + BOARD_WIDTH, score + 1));
        }
    }

    *board.last().unwrap()
}

const VISITED: u8 = 0b001;
const BOTTOM_LEFT: u8 = 0b010;
const TOP_RIGHT: u8 = 0b100;

fn print_board(board: &[u8]) {
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let pos = y * BOARD_WIDTH + x;
            match board[pos] {
                0 => print!(" "),
                1 => print!("."),
                2 => print!("b"),
                3 => print!("B"),
                4 => print!("t"),
                5 => print!("T"),
                6 => print!("!"),
                7 => print!("X"),
                e => print!("{e}"),
            }
        }

        println!();
    }
}

fn mark_neighbours(board: &mut [u8], position: usize) -> Result<(),usize> {
    let mut sweep = HashSet::new();
    let mut flag = 0;
    let x = position % BOARD_WIDTH;
    let y = position / BOARD_WIDTH;

    let window: [(isize, isize); 8] = [
        (-1, -1),  (0, -1),  (1, -1),
        (-1,  0),            (1,  0),
        (-1,  1),  (0,  1),  (1,  1)
    ];

    // inherit flags from surrounding neighbours
    for (x, y) in window
        .iter()
        .map(|&(xoff, yoff)| (x.checked_add_signed(xoff), y.checked_add_signed(yoff)))
        .filter(|(x, y)| x.is_some_and(|x| x < BOARD_WIDTH) && y.is_some_and(|y| y < BOARD_HEIGHT))
    {
        let next = y.unwrap() * BOARD_WIDTH + x.unwrap();
        flag |= board[next] & 0b110;
        if board[next] & VISITED == 1 {
            sweep.insert(next);
        }
    }

    // propagate the flags to all neighbours (and check for complete block)
    for neighbour in sweep {
        if board[neighbour] & flag != flag {
            board[neighbour] |= flag;
            if board[neighbour] == VISITED | TOP_RIGHT | BOTTOM_LEFT {
                return Err(neighbour);
            }
            mark_neighbours(board, neighbour)?;
        }
    }

    Ok(())
}

fn find_first_path_blocker(blocks: &[usize]) -> Option<usize> {
    let mut board = [0u8; BOARD_WIDTH * BOARD_HEIGHT];

    for &b in blocks {
        board[b] |= VISITED;

        if b % BOARD_WIDTH == 0 || b > board.len() - BOARD_HEIGHT {
            board[b] |= BOTTOM_LEFT;
        }

        if b % BOARD_WIDTH == BOARD_WIDTH - 1 || b < BOARD_WIDTH {
            board[b] |= TOP_RIGHT;
        }

        if let Err(_neighbour) = mark_neighbours(&mut board, b) {
            return Some(b);
        }        
    }

    print_board(&board);
    None
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("First argument should be input")
        .into();

    let all_blocks: Vec<usize> = read_to_string(path)
        .expect("Should be able to read from input")
        .lines()
        .filter_map(|l| l.split_once(',').map(|(x, y)|
            BOARD_WIDTH * y
                .parse::<usize>()
                .expect("Y-coordinate should be parsable") +
            x
                .parse::<usize>()
                .expect("X-coordinate should be parsable"))
        )
        .collect();

    if !cfg!(feature = "part2") {
        let blocks = HashSet::from_iter(all_blocks[..1024].iter().copied());
        let answer = find_shortest_path(&blocks);
        println!("Answer: {answer}");
    }
    else if let Some(answer) = find_first_path_blocker(&all_blocks) {
        let x = answer % BOARD_WIDTH;
        let y = answer / BOARD_HEIGHT;
        println!("Answer: {x},{y}");
    }
    else {
        panic!("No solution");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_shortest() {
        let blocks = HashSet::new();
        let answer = find_shortest_path(&blocks);
        assert_ne!(answer, usize::MAX);
    }
}
