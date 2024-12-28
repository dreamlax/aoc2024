use utils::timer::Timer;
use std::collections::{HashSet,VecDeque};
use std::fs::read;
use std::path::PathBuf;

type Score = u32;

#[derive(Eq,PartialEq,Copy,Clone,Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    const fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

const MOVE_PENALTY: Score = 1;
const TURN_PENALTY: Score = 1000;

struct Maze<'a> {
    board: &'a [u8],
    width: usize,
    start: usize,
    end: usize,
}

impl<'a> From<&'a [u8]> for Maze<'a> {
    fn from(m: &'a [u8]) -> Self {
        let width = m
            .iter()
            .position(|ch| *ch == b'\n')
            .map(|w| w + 1)
            .unwrap_or(m.len());
        let start = m
            .iter()
            .position(|ch| *ch == b'S')
            .expect("The maze should have a start");
        let end = m
            .iter()
            .position(|ch| *ch == b'E')
            .expect("The maze should have an end");
        Self {
            board: m,
            width,
            start,
            end
        }
    }
}

impl Maze<'_> {
    fn get_scores(&self) -> Vec<Score> {
        let mut scores: Vec<Score> = vec![Score::MAX; self.board.len()];
        let mut queue: VecDeque<(u32, usize, Direction)> = VecDeque::new();

        queue.push_back((0, self.start, Direction::Right));
        while let Some((cur_score, pos, direction)) = queue.pop_front() {
            if cur_score < scores[pos] && cur_score < scores[self.end] {
                scores[pos] = cur_score;
            }
            else {
                continue;
            }
    
            queue.extend(
                [
                    (Direction::Left, -1isize), 
                    (Direction::Right, 1),
                    (Direction::Down, self.width as isize),
                    (Direction::Up, -(self.width as isize))
                ]
                .iter()
                .filter_map(|(next_direction, offset)| {
                    let next_pos = ((pos as isize) + offset) as usize;
                    if self.board[next_pos] != b'#' && *next_direction != direction.opposite() {
                        let next_score = cur_score + if *next_direction != direction {
                            TURN_PENALTY + MOVE_PENALTY
                        }
                        else {
                            MOVE_PENALTY
                        };
                        Some((next_score, next_pos, *next_direction))
                    }
                    else {
                        None
                    }
                })
            );
        }

        scores
    }

    fn count_best_seats(&self) -> usize {
        let board_len = self.board.len();
        let mut visited = vec![Score::MAX; board_len * 4];
        let mut queue = VecDeque::new();

        // do the same as part 1, but this time we need to keep track of the direction
        // so that we can efficiently backtrack later
        queue.push_back((0, self.start, Direction::Right));
        while let Some((cur_score, pos, direction)) = queue.pop_front() {
            let x = &mut visited[direction as usize * board_len + pos];
            if cur_score < *x {
                *x = cur_score;
            }
            else {
                continue;
            }

            if cur_score > visited[direction as usize * board_len + self.end] || pos == self.end {
                continue;
            }

            queue.extend(
                [
                    (Direction::Left, -1isize), 
                    (Direction::Right, 1),
                    (Direction::Down, self.width as isize),
                    (Direction::Up, -(self.width as isize))
                ]
                .iter()
                .filter_map(|(next_direction, offset)| {
                    let next_pos = ((pos as isize) + offset) as usize;
                    if self.board[next_pos] != b'#' && *next_direction != direction.opposite() {
                        if *next_direction != direction {
                            Some((cur_score + TURN_PENALTY, pos, *next_direction))
                        }
                        else {
                            Some((cur_score + MOVE_PENALTY, next_pos, *next_direction))
                        }
                    }
                    else {
                        None
                    }
                })
            );
        }

        // find the best score at the end
        let best_score = (0..4)
            .map(|i| visited[i * board_len + self.end])
            .min()
            .unwrap();

        // for each of the best scores at the end, add to the queue to start
        // backtracking
        for i in 0..4 {
            if visited[i * board_len + self.end] == best_score {
                queue.push_back((visited[i * board_len + self.end], self.end, match i {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    3 => Direction::Right,
                    _ => panic!("Shouldn't happen")
                }));
            }
        }

        // backtrack from the end taking only the paths that match the move or
        // turn penalty for the next position
        let mut best_seats = HashSet::new();
        while let Some((cur_score, pos, direction)) = queue.pop_front() {
            best_seats.insert(pos);
            
            if pos == self.start {
                break;
            }

            queue.extend(
                [
                    (Direction::Right, -1isize), 
                    (Direction::Left, 1),
                    (Direction::Up, self.width as isize),
                    (Direction::Down, -(self.width as isize))
                ]
                .iter()
                .filter_map(|(next_direction, offset)| {
                    let next_pos = ((pos as isize) + offset) as usize;
                    if self.board[next_pos] != b'#' && *next_direction != direction.opposite() {
                        if *next_direction == direction && visited[*next_direction as usize * board_len + next_pos] == cur_score - MOVE_PENALTY {
                            return Some((cur_score - MOVE_PENALTY, next_pos, *next_direction));
                        }
                        else if *next_direction != direction && visited[*next_direction as usize * board_len + pos] == cur_score - TURN_PENALTY {
                            return Some((cur_score - TURN_PENALTY, pos, *next_direction));
                        }
                    }
                    
                    None
                })
            );
        }

        best_seats.len()
    }
}

fn main() {
    let _timer = Timer::new();
    
    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let input = read(path)
        .expect("Should be able to read from input");

    let maze: Maze = input
        .as_slice()
        .into();

    let answer = if cfg!(feature = "part2") {
        maze.count_best_seats()
    }
    else {
        let scores = maze.get_scores();
        scores[maze.end] as usize
    };

    println!("Answer: {answer}");
}
