use utils::timer::Timer;
use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv
}

impl From<u64> for OpCode {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid OpCode")
        }
    }
}

struct Machine {
    registers: [u64; 3],
    pipeline: Vec<u64>,
    ip: usize
}

#[derive(Debug)]
struct MachineParseError(&'static str);

impl std::fmt::Display for MachineParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Machine {
    type Err = MachineParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s
            .split(|c: char| matches!(c, ':' | '\n' | ','))
            .collect();

        let registers: [u64; 3] = [
            input[1].trim().parse().map_err(|_| MachineParseError("Register A"))?,
            input[3].trim().parse().map_err(|_| MachineParseError("Register B"))?,
            input[5].trim().parse().map_err(|_| MachineParseError("Register C"))?
        ];

        let pipeline = input[8..]
            .iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>,_>>()
            .map_err(|_| MachineParseError("Program input"))?;

        Ok(Self {
            registers,
            pipeline,
            ip: 0
        })
    }
}

impl Machine {
    fn get_combo_operand(&self) -> u64 {
        let val = self.pipeline[self.ip + 1];
        match val {
            0..=3 => val,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("Invalid operand")
        }
    }

    fn run(&mut self) -> Vec<u64> {
        let mut result = Vec::new();

        while self.ip < self.pipeline.len() {
            let op_code: OpCode = self.pipeline[self.ip].into();

            match op_code {
                OpCode::Adv => {
                    self.registers[0] >>= self.get_combo_operand();
                },
                OpCode::Bxl => {
                    self.registers[1] ^= self.pipeline[self.ip+1];
                },
                OpCode::Bst => {
                    self.registers[1] = self.get_combo_operand() % 8;
                },
                OpCode::Jnz => {
                    if self.registers[0] != 0 {
                        self.ip = self.pipeline[self.ip+1] as usize;
                        continue;
                    }
                },
                OpCode::Bxc => {
                    self.registers[1] ^= self.registers[2];
                },
                OpCode::Out => {
                    result.push(self.get_combo_operand() % 8);
                },
                OpCode::Bdv => {
                    self.registers[1] = self.registers[0] >> self.get_combo_operand();
                },
                OpCode::Cdv => {
                    self.registers[2] = self.registers[0] >> self.get_combo_operand();
                }
            }

            self.ip += 2;
        }

        result
    }
}

fn get_quine(a: u64, m: &mut Machine, i: usize, possibilities: &mut BTreeSet<u64>) -> Option<u64> {
    for b in 0..8 {
        let a = a << 3 | b;
        m.registers = [ a, 0, 0 ];
        m.ip = 0;
        
        if m.run() == m.pipeline[i..] {
            if i == 0 {
                possibilities.insert(a);
            }
            else {
                let answer = get_quine(a, m, i-1, possibilities);
                if answer.is_some() {
                    return answer;
                }
            }
        }
    }

    possibilities.first().copied()
}

fn main() {
    let _timer = Timer::new();
    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Provide input filename as argument")
        .into();

    let mut machine: Machine = read_to_string(path)
        .expect("Should be able to read from input")
        .parse()
        .expect("Input is not parsable");

    if cfg!(feature = "part2") {
        let mut possibilities = BTreeSet::new();
        let i = machine.pipeline.len() - 1;
        let result = get_quine(0, &mut machine, i, &mut possibilities)
            .expect("There should be a solution");
        println!("Answer: {result}");
    }
    else {
        let result = machine.run();
        println!("Answer: {:?}", result);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let _machine: Machine = "Register A: 1\nRegister B: 2\nRegister C: 3\n\nProgram: 1, 2, 3, 4, 5, 6\n"
            .parse()
            .expect("Should be able to parse input");
    }

    #[test]
    fn test_example_1() {
        let mut machine = Machine {
            registers: [0, 0, 9],
            pipeline: vec![2, 6],
            ip: 0
        };

        machine.run();

        assert_eq!(machine.registers[1], 1, "Register B should be 1");
    }
}
