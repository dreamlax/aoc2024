use utils::timer::Timer;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

const BOARD_WIDTH: usize = 101;
const BOARD_HEIGHT: usize = 103;

struct Robot {
    position: [usize; 2],
    vector: [isize; 2]
}

impl Robot {
    fn get_quadrant(&self) -> Option<usize> {
        const HALF_WIDTH: usize = BOARD_WIDTH / 2;
        const HALF_HEIGHT: usize = BOARD_HEIGHT / 2;

        if self.position[0] == HALF_WIDTH || self.position[1] == HALF_HEIGHT {
            return None;
        }

        let qx = self.position[0] / (HALF_WIDTH + 1);
        let qy = self.position[1] / (HALF_HEIGHT + 1);
        
        Some(qy * 2 + qx)
    }

    fn make_move(mut self, seconds: usize) -> Self {
        self.position[0] += (BOARD_WIDTH * seconds).saturating_add_signed(self.vector[0] * seconds as isize);
        self.position[1] += (BOARD_HEIGHT * seconds).saturating_add_signed(self.vector[1] * seconds as isize);
        self.position[0] %= BOARD_WIDTH;
        self.position[1] %= BOARD_HEIGHT;
        self
    }

    fn make_move_once(&mut self) {
        self.position[0] += (BOARD_WIDTH as isize + self.vector[0]) as usize;
        self.position[1] += (BOARD_HEIGHT as isize + self.vector[1]) as usize;
        self.position[0] %= BOARD_WIDTH;
        self.position[1] %= BOARD_HEIGHT;
    }
}

#[derive(Debug)]
struct RobotParseError;

impl FromStr for Robot {
    type Err = RobotParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(['=', ',', ' '])
            .enumerate()
            .filter_map(|(idx, s)| if matches!(idx, 1 | 2 | 4 | 5) { Some(s) } else { None })
            .map(str::parse)
            .collect::<Result<Vec<isize>,_>>()
            .map_err(|_| RobotParseError)?;

        Ok(Self {
            position: [values[0] as usize, values[1] as usize],
            vector: [values[2], values[3]]
        })
    }
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let input = read_to_string(path)
        .expect("Should be able to read from input");

    let robots = input
        .lines()
        .map(Robot::from_str)
        .map(Result::unwrap);

    let answer = if !cfg!(feature = "part2") {
        robots
            .map(|r| r.make_move(100))
            .fold([0, 0, 0, 0], |mut quadrants, r| {
                if let Some(q) = r.get_quadrant() {
                    quadrants[q] += 1;
                }
                quadrants
            })
            .iter()
            .product()
    }
    else {
        let mut robots: Vec<Robot> = robots.collect();
        let mut min = (usize::MAX, 0);

        // after BOARD_WIDTH * BOARD_HEIGHT iterations, the board should
        // "reset" itself (I think), so that should be the upper bound
        for seconds in 1..=(BOARD_WIDTH * BOARD_HEIGHT) {
            robots
                .iter_mut()
                .for_each(|r| r.make_move_once());
            
            let safety_score: usize = robots
                .iter()
                .fold([0, 0, 0, 0], |mut quadrants, r| {
                    if let Some(q) = r.get_quadrant() {
                        quadrants[q] += 1;
                    }
                    quadrants
                })
                .iter()
                .product();

            if safety_score < min.0 {
                min.0 = safety_score;
                min.1 = seconds;

                if cfg!(feature = "print_tree") {
                    let mut grid = [0usize; BOARD_WIDTH * BOARD_HEIGHT];
                    for r in &robots {
                        grid[r.position[1] * BOARD_WIDTH + r.position[0]] += 1;
                    }

                    for y in 0..BOARD_HEIGHT {
                        for x in 0..BOARD_WIDTH {
                            let ch = match grid[y * BOARD_WIDTH + x] {
                                0 => " ",
                                1 => "X",
                                2 => "\x1B[1;31mX\x1B[0m",
                                3..6 => "\x1B[1;33mX\x1B[0m",
                                _ => "\x1B[1;34mX\x1B[0m",
                            };

                            print!("{ch}");
                        }
                        println!();
                    }
                    println!("---------------------------------------");
                }
            }
        }

        assert!(min.1 > 0, "Should be a minimum!");
        min.1
    };

    println!("Answer: {answer}");
}
