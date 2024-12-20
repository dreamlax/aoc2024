use utils::timer::Timer;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
struct ClawMachine {
    m: [[i64; 2]; 2],
    c: [i64; 2]
}

impl ClawMachine {
    fn count_tokens(&self) -> usize {
        let determinant = self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0];
        let inverse = [[self.m[1][1], -self.m[0][1]], [-self.m[1][0], self.m[0][0]]];

        let x = inverse[0][0] * self.c[0] + inverse[0][1] * self.c[1];
        let y = inverse[1][0] * self.c[0] + inverse[1][1] * self.c[1];

        let (x, x_rem) = (x / determinant, x % determinant);
        let (y, y_rem) = (y / determinant, y % determinant);

        if x_rem != 0 || y_rem != 0 || x < 0 || y < 0 {
            return 0;
        }
        
        x as usize * 3 + y as usize
    }
}

#[derive(Debug)]
struct ClawMachineParseError;

impl FromStr for ClawMachine {
    type Err = ClawMachineParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .lines()
            .flat_map(|line| line
                .split(['+', '=', ','])
                .enumerate()
                .filter_map(|(idx, s)| if idx == 1 || idx == 3 { Some(s) } else { None })
            )
            .map(str::parse)
            .collect::<Result<Vec<i64>,_>>()
            .map_err(|_| ClawMachineParseError)?;

        let constant = if cfg!(feature = "part2") {
            10_000_000_000_000
        } else {
            0
        };

        Ok(Self {
            m: [ [ values[0], values[2] ], [ values[1], values[3] ] ],
            c: [ values[4] + constant, values[5] + constant ]
        })
    }
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let total_tokens: usize = read_to_string(&path)
        .expect("Should be able to read input")
        .split("\n\n")
        .map(ClawMachine::from_str)
        .map(Result::unwrap)
        .map(|c| c.count_tokens())
        .sum();
        
    println!("Answer: {total_tokens}");
}
