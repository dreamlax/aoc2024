use std::fmt::Display;

use crate::board::{Board, InstructionProcessor};

#[derive(Eq,PartialEq)]
pub enum Cell {
    Empty,
    Wall,
    Box
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Self::Wall,
            b'O' => Self::Box,
            _ => Self::Empty
        }
    }
}

pub struct Part1Board {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub robot: usize,
}

impl From<&[u8]> for Part1Board {
    fn from(value: &[u8]) -> Self {
        let width = value
            .iter()
            .position(|ch| *ch == b'\n')
            .map(|w| w + 1)
            .unwrap_or(value.len());
        let robot = value
            .iter()
            .position(|ch| *ch == b'@')
            .expect("There should be a robot on this map");

        Part1Board {
            cells: value
                .iter()
                .map(|&ch| ch.into())
                .collect(),
            width,
            robot
        }
    }
}

impl Display for Part1Board {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut x = 0;
		for c in &self.cells {
			write!(f, "{}", match c {
				Cell::Empty => ".",
				Cell::Wall => "#",
				Cell::Box => "O",
			})?;
			x += 1;
			if x % self.width == 0 {
				write!(f, "\n")?;
			}
		}
		Ok(())
	}
}

impl Board<'_> for Part1Board {
    fn sum_gps(&self) -> usize {
        self.cells.iter()
            .enumerate()
            .filter(|(_idx, cell)| **cell == Cell::Box)
            .map(|(idx, _cell)| idx / self.width * 100 + idx % self.width)
            .sum()
    }
}

impl Part1Board {
    fn relative_move(&mut self, offset: isize) {
        let mut next = (self.robot as isize + offset) as usize;

        while self.cells[next] != Cell::Wall {
            if self.cells[next] == Cell::Empty {
                self.robot = (self.robot as isize + offset) as usize;
                self.cells.swap(next, self.robot);
                return;
            }

            next = (next as isize + offset) as usize;
        }
    }
}

impl InstructionProcessor for Part1Board {
    fn move_left(&mut self) {
        self.relative_move(-1);
    }

	fn move_right(&mut self) {
        self.relative_move(1);
	}

	fn move_up(&mut self) {
		self.relative_move(-(self.width as isize));
	}

	fn move_down(&mut self) {
		self.relative_move(self.width as isize);
	}
}
