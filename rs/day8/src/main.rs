use utils::timer::Timer;
use std::collections::{HashMap,HashSet};
use std::fs::read;
use std::path::PathBuf;

fn get_antennae(board: &[u8]) -> HashMap<u8, Vec<usize>> {
    board
        .iter()
        .enumerate()
        .filter(|(_idx, ch)| ch.is_ascii_alphanumeric())
        .fold(HashMap::new(), |mut acc, (idx, ch)| {
            acc.entry(*ch).or_default().push(idx);
            acc
        })
}

fn count_antinodes(board: &[u8], antennae: &HashMap<u8, Vec<usize>>) -> usize {
    let board_width = board
        .iter()
        .position(|ch| *ch == b'\n')
        .map(|x| x + 1)
        .unwrap_or_else(|| board.len());

    let antinodes: HashSet<usize> = HashSet::from_iter(antennae
        .iter()
        .flat_map(|(_ch, points)| {
            points[0..points.len()-1]
                .iter()
                .enumerate()
                .flat_map(|(idx, p1)|
                    points[idx+1..points.len()]
                        .iter()
                        .flat_map(|p2| {
                            let diff = *p2 - *p1;
                            let x1 = (*p1 % board_width) as isize;
                            let x2 = (*p2 % board_width) as isize;
                            let dx = x2 - x1;
                            
                            [
                                p1
                                    .checked_sub(diff)
                                    .filter(|_| x1 - dx >= 0 && x1 - dx < board_width as isize),
                                p2
                                    .checked_add(diff)
                                    .filter(|_| x2 + dx >= 0 && x2 + dx < board_width as isize)
                            ]
                        })
                )
        })
        .filter(|p| p.is_some_and(|p| p < board.len() && board[p] != b'\n'))
        .map(|p| p.unwrap())
    );

    antinodes.len()
}

fn count_extended_antinodes(board: &[u8], antennae: &HashMap<u8, Vec<usize>>) -> usize {
    let board_width = board
        .iter()
        .position(|ch| *ch == b'\n')
        .map(|x| x + 1)
        .unwrap_or_else(|| board.len());

    let antinodes: HashSet<usize> = HashSet::from_iter(antennae
        .iter()
        .flat_map(|(_ch, points)| {
            points[0..points.len()-1]
                .iter()
                .enumerate()
                .flat_map(|(idx, p1)|
                    points[idx+1..points.len()]
                        .iter()
                        .flat_map(|p2| {
                            let diff = *p2 - *p1;
                            let x1 = (*p1 % board_width) as isize;
                            let x2 = (*p2 % board_width) as isize;
                            let dx = x2 - x1;
                            let mut points = vec![Some(*p1), Some(*p2)];
                            let mut b = x1 - dx;
                            let mut x = *p1;
                            while b >= 0 && b < board_width as isize {
                                let Some(prev) = x.checked_sub(diff) else {
                                    break;
                                };
                                points.push(Some(prev));
                                b -= dx;
                                x = prev;
                            }

                            b = x2 + dx;
                            x = *p2;
                            while b >= 0 && b < board_width as isize {
                                let next = x + diff;
                                if next >= board.len() {
                                    break;
                                }
                                points.push(Some(next));
                                b += dx;
                                x = next;
                            }

                            points
                        })
                )
        })
        .filter(|p| p.is_some_and(|p| p < board.len() && board[p] != b'\n'))
        .map(|p| p.unwrap())
    );

    antinodes.len()
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let board = read(path)
        .expect("Should be able to read from path");

    let antennae = get_antennae(&board);
    let answer = if cfg!(feature = "part2") {
        count_extended_antinodes(&board, &antennae)
    }
    else {
        count_antinodes(&board, &antennae)
    };

    println!("Answer: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let board = b"....A..A.....\n";
        let antennae = get_antennae(board);
        let antinodes = count_antinodes(board, &antennae);
        assert_eq!(antinodes, 2, "There should be 2 antinodes");
    }

    #[test]
    fn test_overlapping_antinode() {
        let board = b"....A..A.....B..B.....\n";
        let antennae = get_antennae(board);
        let antinodes = count_antinodes(board, &antennae);
        assert_eq!(antinodes, 3, "There should be 3 antinodes");
    }

    #[test]
    fn test_antinode_on_antenna() {
        let board = b"....A..A..B..B.....\n";
        // antinodes  .#.....#..#.....#..
        let antennae = get_antennae(board);
        let antinodes = count_antinodes(board, &antennae);
        assert_eq!(antinodes, 4, "There should be 4 antinodes");
    }

    #[test]
    fn test_one_out_of_bounds() {
        let board = b"..A....A........\n";
        // antinodes  ............#...
        let antennae = get_antennae(board);
        let antinodes = count_antinodes(board, &antennae);
        assert_eq!(antinodes, 1, "There should be 1 antinode");
    }

    #[test]
    fn test_neighbouring() {
        let board = b"..aa..\n";
        // antinodes  .#..#.
        let antennae = get_antennae(board);
        let antinodes = count_antinodes(board, &antennae);
        assert_eq!(antinodes, 2, "There should be 2 antinodes");
    }

    #[test]
    fn test_double_neighbouring() {
        let board = b".xaax.\n";
        // antinodes  .#..#.
        let antennae = get_antennae(board);
        let antinodes = count_antinodes(board, &antennae);
        assert_eq!(antinodes, 2, "There should be 2 antinodes");
    }

    #[test]
    fn test_single_neighbouring_single_oob() {
        let board = b".aa\n";
        // antinodes  #..
        let antennae = get_antennae(board);
        let antinodes = count_antinodes(board, &antennae);
        assert_eq!(antinodes, 1, "There should be 1 antinodes");
    }

    #[test]
    fn test_single_neighbouring_single_oob_2() {
        let board = b"aa.\n";
        // antinodes  ..#
        let antennae = get_antennae(board);
        let antinodes = count_antinodes(board, &antennae);
        assert_eq!(antinodes, 1, "There should be 1 antinodes");
    }

    #[test]
    fn test_multiline() {
        let board = b"\
............
............
............
............
............
......A.....
............
............
........A...
............
............
............
";
        let antennae = get_antennae(board);
        let antinodes = count_antinodes(board, &antennae);
        assert_eq!(antinodes, 2, "There should be 2 antinodes");
    }

    #[test]
    fn test_extended_antinodes() {
        let board = b"\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";
        let antennae = get_antennae(board);
        let antinodes = count_extended_antinodes(board, &antennae);
        assert_eq!(antinodes, 9, "There should be 9 antinodes");
    }
    
    #[test]
    fn test_extended_antinodes_backwards() {
        let board = b"\
.........T
......T...
........T.
..........
..........
..........
..........
..........
..........
..........
";
        let antennae = get_antennae(board);
        let antinodes = count_extended_antinodes(board, &antennae);
        assert_eq!(antinodes, 9, "There should be 9 antinodes");
    }

    #[test]
    fn test_extended_vertical() {
        let board = b"\
..........
..........
..........
...T......
..........
...T......
..........
..........
..........
..........
";
        let antennae = get_antennae(board);
        let antinodes = count_extended_antinodes(board, &antennae);
        assert_eq!(antinodes, 5, "There should be 5 antinodes");
    }
    
    #[test]
    fn test_extended_vertical_neighbouring() {
        let board = b"\
..........
..........
..........
...T......
...T......
..........
..........
..........
..........
..........
";
        let antennae = get_antennae(board);
        let antinodes = count_extended_antinodes(board, &antennae);
        assert_eq!(antinodes, 10, "There should be 10 antinodes");
    }
    
    #[test]
    fn test_extended_horizontal() {
        let board = b"\
..........
..........
..........
...T.T....
..........
..........
..........
..........
..........
..........
";
        let antennae = get_antennae(board);
        let antinodes = count_extended_antinodes(board, &antennae);
        assert_eq!(antinodes, 5, "There should be 5 antinodes");
    }
    
    #[test]
    fn test_extended_horizontal_neighbouring() {
        let board = b"\
..........
..........
..........
...TT.....
..........
..........
..........
..........
..........
..........
";
        let antennae = get_antennae(board);
        let antinodes = count_extended_antinodes(board, &antennae);
        assert_eq!(antinodes, 10, "There should be 10 antinodes");
    }
    
    #[test]
    fn test_extended_horizontal_neighbouring_noise() {
        let board = b"\
..........
..........
..........
...TT.....
..........
..........
.....p....
..........
..........
..........
";
        let antennae = get_antennae(board);
        let antinodes = count_extended_antinodes(board, &antennae);
        assert_eq!(antinodes, 10, "There should be 10 antinodes");
    }
}
