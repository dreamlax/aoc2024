use std::fs::read;
use std::path::PathBuf;

use utils::timer::Timer;

struct RaceTrack<'a> {
    board: &'a [u8],
    width: usize,
    start: usize,
    end: usize,
}

impl<'a> From<&'a [u8]> for RaceTrack<'a> {
    fn from(board: &'a [u8]) -> Self {
        let width = board
            .into_iter()
            .position(|ch| *ch == b'\n')
            .map(|w| w + 1)
            .unwrap_or_else(|| board.len());

        let start = board
            .into_iter()
            .position(|ch| *ch == b'S')
            .expect("There should be a start");

        let end = board
            .into_iter()
            .position(|ch| *ch == b'E')
            .expect("There should be an end");

        RaceTrack {
            board,
            width,
            start,
            end
        }
    }
}

impl RaceTrack<'_> {
    fn get_scores(&self) -> (Vec<usize>, usize) {
        let mut score_board = vec![usize::MAX; self.board.len()];
    
        let mut score = 0;
        let mut pos = self.start;

        while pos != self.end {
            score_board[pos] = score;
            score += 1;

            pos = [
                    pos - self.width,
                    pos + self.width,
                    pos - 1,
                    pos + 1
                ]
                .into_iter()
                .filter(|&next| matches!(self.board[next], b'.' | b'E') && score_board[next] == usize::MAX)
                .nth(0)
                .expect("Race is incomplete");
        }

        score_board[self.end] = score + 1;
        (score_board, score)
    }
    
    fn find_shortcuts(&self, score_board: &[usize], target_saving: usize) -> usize{
        let mut count = 0;
    
        let mut pos = self.start;
        let iwidth = self.width as isize;

        while pos != self.end {
            count += [
                    -2,
                    -iwidth - 1,
                    -(iwidth * 2),
                    -iwidth + 1,
                    2,
                    iwidth + 1,
                    iwidth * 2,
                    iwidth - 1
                ]
                .iter()
                .map(|offset| pos.checked_add_signed(*offset))
                .filter(|new_pos| new_pos.is_some_and(|v| v < self.board.len() && self.board[v] != b'\n'))
                .map(Option::unwrap)
                .filter(|new_pos| matches!(self.board[*new_pos], b'.' | b'E') && score_board[pos] + 2 < score_board[*new_pos])
                .map(|new_pos| score_board[new_pos] - score_board[pos] - 2)
                .filter(|saving| *saving >= target_saving)
                .count();

            pos = [
                    pos - self.width,
                    pos + self.width,
                    pos - 1,
                    pos + 1
                ]
                .into_iter()
                .filter(|&next| matches!(self.board[next], b'.' | b'E') && score_board[next] > score_board[pos])
                .nth(0)
                .expect("Race is incomplete again");
        }

        count
    }
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Input file should be given as first argument")
        .into();

    let input = read(path)
        .expect("Should be able to read from input");

    let race_track = RaceTrack::from(input.as_slice());

    let (score_board, _target_time) = race_track.get_scores();
    let answer = race_track.find_shortcuts(&score_board, 100);

    println!("Answer: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_case() {
        let input = b"\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

        let race_track = RaceTrack::from(&input[..]);
        let (scores, _target) = race_track.get_scores();
        let answer = race_track.find_shortcuts(&scores, 64);
        
        assert_eq!(answer, 1);
    }
}
