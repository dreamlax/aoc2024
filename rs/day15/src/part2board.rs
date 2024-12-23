use std::fmt::Display;
use std::collections::BTreeSet;
use crate::board::{Board, InstructionProcessor};

#[derive(Eq,PartialEq)]
pub enum Cell {
	Empty,
	Wall,
	BoxLeft,
	BoxRight
}

pub struct Part2Board {
	cells: Vec<Cell>,
	width: usize,
	robot: usize,
}

impl Display for Part2Board {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut x = 0;
		for c in &self.cells {
			if x == self.robot {
				write!(f, "@")?;
			}
			else {
				write!(f, "{}", match c {
					Cell::Empty => ".",
					Cell::Wall => "#",
					Cell::BoxLeft => "[",
					Cell::BoxRight => "]"
				})?;
			}
			x += 1;
			if x % self.width == 0 {
				write!(f, "\n")?;
			}
		}
		Ok(())
	}
}

impl From<&[u8]> for Part2Board {
	fn from(value: &[u8]) -> Self {
		let width = value
			.iter()
			.position(|ch| *ch == b'\n')
			.map(|w| (w + 1) * 2)
			.unwrap_or(value.len() * 2);

		let robot = value
			.iter()
			.position(|ch| *ch == b'@')
			.map(|r| r * 2)
			.expect("There should be a robot on this map");

		let cells = value
			.iter()
			.flat_map(|ch| match ch {
				b'#' => [ Cell::Wall, Cell::Wall ],
				b'O' => [ Cell::BoxLeft, Cell::BoxRight ],
				_ => [ Cell::Empty, Cell::Empty ],
			})
			.collect::<Vec<Cell>>();

		Self {
			cells,
			width,
			robot
		}
	}
}

impl Board<'_> for Part2Board {
	fn sum_gps(&self) -> usize {
		self.cells
			.iter()
			.enumerate()
			.filter(|(_idx, cell)| **cell == Cell::BoxLeft)
			.map(|(idx, _cell)| idx / self.width * 100 + idx % self.width)
			.sum()
	}
}

impl Part2Board {
	fn find_boxes(&self, position: usize, offset: isize, boxes: &mut BTreeSet<usize>) -> Result<(),usize> {
		Ok(match self.cells[position] {
			Cell::BoxLeft => {
				boxes.insert(position);
				boxes.insert(position + 1);
				self.find_boxes((position as isize + offset) as usize, offset, boxes)?;
				self.find_boxes((position as isize + offset) as usize + 1, offset, boxes)?;
			},
			Cell::BoxRight => {
				boxes.insert(position);
				boxes.insert(position - 1);
				self.find_boxes((position as isize + offset) as usize, offset, boxes)?;
				self.find_boxes((position as isize + offset) as usize - 1, offset, boxes)?;
			},
			Cell::Wall => {
				return Err(position);
			},
			Cell::Empty => ()
		})
	}
}

impl InstructionProcessor for Part2Board {
	fn move_left(&mut self) {
		let mut x = self.robot - 1;
		while self.cells[x] != Cell::Wall {
			if self.cells[x] == Cell::Empty {
				self.robot -= 1;
				for i in x..self.robot {
					self.cells.swap(i, i + 1);
				}
				return;
			}
			x -= 1;
		}
	}

	fn move_right(&mut self) {
		let mut x = self.robot + 1;
		while self.cells[x] != Cell::Wall {
			if self.cells[x] == Cell::Empty {
				for i in (self.robot..x).rev() {
					self.cells.swap(i, i + 1);
				}
				self.robot += 1;
				return;
			}
			x += 1;
		}
	}

	fn move_up(&mut self) {
		let mut boxes = BTreeSet::new();
		if let Err(_wall) = self.find_boxes(self.robot - self.width, -(self.width as isize), &mut boxes) {
			return;
		}

		for b in boxes.iter() {
			self.cells.swap(*b, b - self.width);
		}

		self.robot -= self.width;
	}

	fn move_down(&mut self) {
		let mut boxes = BTreeSet::new();
		if let Err(_wall) = self.find_boxes(self.robot + self.width, self.width as isize, &mut boxes) {
			return;
		}

		for b in boxes.iter().rev() {
			self.cells.swap(b + self.width, *b);
		}

		self.robot += self.width;
	}
}

#[cfg(test)]
mod tests {
	use crate::board::InstructionReader;
    use super::*;

	#[test]
	fn test_move_left() {
		let input = b"#..O.@.O..#";
		//          b"##....[]..@...[]....##\n";
		let mut board: Part2Board = input.as_slice().into();

		for &instruction in b"<<<<<" {
			board.process_instruction(instruction);
		}
		
		let result = board.to_string();
		assert_eq!(result, "##.[]@........[]....##\n");
	}


	#[test]
	fn test_move_right() {
		let input = b"#..O.@.O..#";
		//          b"##....[]..@...[]....##\n"
		let mut board: Part2Board = input.as_slice().into();

		for &instruction in b">>>>>" {
			board.process_instruction(instruction);
		}
		
		let result = board.to_string();
		assert_eq!(result, "##....[].......@[]..##\n");
	}

	#[test]
	fn test_move_up() {
		let input = b"\
########
#.O....#
#......#
#.O....#
#.@....#
########
";
		let mut board: Part2Board = input.as_slice().into();

		for instruction in b">>^<v<^^" {
			board.process_instruction(*instruction);
		}

		assert_eq!(board.robot, 58);
	}
}
