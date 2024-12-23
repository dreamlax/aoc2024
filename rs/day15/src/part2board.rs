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
		unimplemented!();
	}
}

impl InstructionProcessor for Part2Board {
	fn move_left(&mut self) {
		unimplemented!();
	}

	fn move_right(&mut self) {
		unimplemented!();
	}

	fn move_up(&mut self) {
		unimplemented!();
	}

	fn move_down(&mut self) {
		unimplemented!();
	}
}
