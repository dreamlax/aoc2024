use std::fmt::Display;

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

pub struct Board {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub robot: usize,
}

impl From<&[u8]> for Board {
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

        Board {
            cells: value
                .iter()
                .map(|&ch| ch.into())
                .collect(),
            width,
            robot
        }
    }
}

impl Display for Board {
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

impl Board {
    pub fn move_left(&mut self) {
        let mut x = self.robot - 1;
        while self.cells[x] != Cell::Wall {
            if self.cells[x] == Cell::Empty {
                self.cells.swap(x, self.robot - 1);
				self.robot -= 1;
				return;
            }
			x -= 1;
        }
    }

	pub fn move_right(&mut self) {
        let mut x = self.robot + 1;
        while self.cells[x] != Cell::Wall {
            if self.cells[x] == Cell::Empty {
                self.cells.swap(x, self.robot + 1);
				self.robot += 1;
				return;
            }
			x += 1;
        }
	}

	pub fn move_up(&mut self) {
		let mut y = self.robot - self.width;
		while self.cells[y] != Cell::Wall {
			if self.cells[y] == Cell::Empty {
				self.cells.swap(y, self.robot - self.width);
				self.robot -= self.width;
				return;
			}
			y -= self.width;
		}
	}

	pub fn move_down(&mut self) {
		let mut y = self.robot + self.width;
		while self.cells[y] != Cell::Wall {
			if self.cells[y] == Cell::Empty {
				self.cells.swap(y, self.robot + self.width);
				self.robot += self.width;
				return;
			}
			y += self.width;
		}
	}

    pub fn process_instruction(&mut self, instruction: u8) {
        match instruction {
            b'<' => self.move_left(),
			b'>' => self.move_right(),
			b'^' => self.move_up(),
			b'v' => self.move_down(),
			_ => ()
        }
    }
}
