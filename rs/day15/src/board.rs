pub trait InstructionReader {
	fn process_instruction(&mut self, instruction: u8);
}

pub trait InstructionProcessor {
	fn move_left(&mut self);
	fn move_right(&mut self);
	fn move_up(&mut self);
	fn move_down(&mut self);
}

impl<T: InstructionProcessor> InstructionReader for T {
	fn process_instruction(&mut self, instruction: u8) {
		match instruction {
			b'<' => self.move_left(),
			b'>' => self.move_right(),
			b'^' => self.move_up(),
			b'v' => self.move_down(),
			_ => ()
		}
	}
}

pub trait Board<'a> : From<&'a [u8]> + InstructionReader + InstructionProcessor {
	fn sum_gps(&self) -> usize;
}
