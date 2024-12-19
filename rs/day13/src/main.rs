use utils::timer::Timer;
use std::collections;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
struct Equation {
    coeff_m: u32,
    coeff_n: u32,
    sum: u32
}

impl Equation {
    fn find_intersection(&self, other: &Self) -> Option<(u32, u32)> {
        None
    }
}

#[derive(Debug)]
struct ClawMachine {
    equations: [Equation; 2]
}

impl ClawMachine {
    fn count_tokens(&self) -> usize {
        0
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
            .collect::<Result<Vec<u32>,_>>()
            .map_err(|_| ClawMachineParseError)?;

        Ok(Self {
            equations: [
                Equation { coeff_m: values[0], coeff_n: values[1], sum: values[4] },
                Equation { coeff_m: values[2], coeff_n: values[3], sum: values[5] },
            ]
        })
    }
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let claw_machines = read_to_string(&path)
        .expect("Should be able to read input")
        .split("\n\n")
        .map(ClawMachine::from_str)
        .collect::<Result<Vec<ClawMachine>,_>>()
        .expect("Should be able to parse claw machines");

    dbg!(claw_machines);
}
