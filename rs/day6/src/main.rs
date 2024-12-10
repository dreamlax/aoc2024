use utils::timer::Timer;
use std::fs::read;
use std::ops::{BitAnd, BitOrAssign};
use std::path::PathBuf;

#[repr(u8)]
#[derive(PartialEq,Copy,Clone)]
enum Direction {
    Up = 0x08,
    Down = 0x01,
    Left = 0x02,
    Right = 0x04
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up
        }
    }
}

const VISITED: u8 = 0x80;

impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        value as u8
    }
}

fn get_next_position(board: &[u8], board_width: usize, current: usize, mut cur_direction: Direction) -> Option<(usize, Direction)> {
    loop {
        let Some(next_attempt) = (match cur_direction {
            Direction::Up => current.checked_sub(board_width),
            Direction::Down => current.checked_add(board_width),
            Direction::Left => current.checked_sub(1),
            Direction::Right => current.checked_add(1),
        }) else {
            // we've underwrapped, so we've left the board
            return None;
        };

        if next_attempt >= board.len() || board[next_attempt] == b'\n' {
            // we've overshot, so we've left the board
            // (we can use the linefeed character to detect an overshot)
            return None;
        }

        match board[next_attempt] {
            b'#' => {
                cur_direction = cur_direction.rotate();
            },
            b'.' | b'X' => {
                return Some((next_attempt, cur_direction));
            },
            _ => panic!("Should not occur!")
        };
    }
}

fn trace_guard(board: &mut [u8]) -> usize {
    let board_width = 1 + board
        .iter()
        .position(|ch| *ch == b'\n')
        .expect("Invalid input");
    let starting_position = board
        .iter()
        .position(|ch| *ch == b'^')
        .expect("Missing starting position");

    // simplify logic a bit
    board[starting_position] = b'.';
    let mut cur_direction = Direction::Up;
    let mut steps = 0usize;
    let mut current = starting_position;

    loop {
        match board[current] {
            b'.' => {
                board[current] = b'X';
                steps += 1;
            },
            b'X' => {
            },
            _ => {
                panic!("Shouldn't happen!");
            }
        }

        let Some((next, next_direction)) = get_next_position(board, board_width, current, cur_direction) else {
            break;
        };

        current = next;
        cur_direction = next_direction;
    }

    steps
}

fn count_loop_points(board: &mut [u8]) -> usize {
    let board_width = 1 + board
        .iter()
        .position(|ch| *ch == b'\n')
        .expect("Invalid input");
    let mut starting_position = board
        .iter()
        .position(|ch| *ch == b'^')
        .expect("Missing starting position");

    let mut starting_direction = Direction::Up;
    let mut loops = 0usize;

    // can't put a barrier on the starting position
    board[starting_position] = b'X';

    'outer: loop {
        let mut path = vec![0u8; board.len()];
        let mut current = starting_position;
        let mut cur_direction = starting_direction;

        let Some((barrier_position, next_direction)) = get_next_position(board, board_width, current, cur_direction) else {
            // nowhere more to test
            break 'outer;
        };

        if board[barrier_position] == b'X' {
            // if we've already been here, we can't put a barrier here
            starting_position = barrier_position;
            starting_direction = next_direction;
            continue 'outer;
        }

        // simulate the barrier
        board[barrier_position] = b'#';

        'main: loop {
            if path[current].bitand(VISITED) > 0 && path[current].bitand(cur_direction as u8) > 0 {
                loops += 1;
                break 'main;
            }

            path[current].bitor_assign(VISITED);
            path[current].bitor_assign(cur_direction as u8);

            let Some((next, next_direction)) = get_next_position(board, board_width, current, cur_direction) else {
                break 'main;
            };
            
            current = next;
            cur_direction = next_direction;
        }

        board[barrier_position] = b'X';
        starting_position = barrier_position;
        starting_direction = next_direction;
    }

    loops
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .skip(1)
        .next()
        .expect("Should have file argument")
        .into();

    let mut input = read(path)
        .expect("Should be able to read from path");

    if !cfg!(feature = "part2") {
        let unique_steps = trace_guard(&mut input);
        println!("Answer: {unique_steps}");
    }
    else {
        let loop_points = count_loop_points(&mut input);
        println!("Answer: {loop_points}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corner_trace_guard() {
        let mut test = Vec::from(b"\
.#....
....#.
...#..
.^....
");
        assert_eq!(trace_guard(&mut test), 6);
    }

    #[test]
    fn test_corner_count_loops() {
        let mut test = Vec::from(b"\
.#....
....#.
...#..
.^....
");
        let count = count_loop_points(&mut test);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_backwards_count_loops() {
        let mut test = Vec::from(b"\
..#.#.
.....#
....#.
..^...
");
        let count = count_loop_points(&mut test);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_nothing() {
        let mut test = Vec::from(b"\
.###..
.#.#..
.#.#..
.#^#..
");
        let count = count_loop_points(&mut test);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_no_cross_loop() {
        let mut test = Vec::from(b"\
......
...#..
....#.
......
.^#...
...#..
");
        let count = count_loop_points(&mut test);
        assert_eq!(count, 1);
    }
}
