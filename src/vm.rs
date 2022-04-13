use std::io::Write;

use console::Term;

pub fn interpret(code: Vec<u8>) -> Result<(), Box<dyn std::error::Error>>
{
	const DATA_SIZE: usize = 65536;

	let data: Box<[u8; DATA_SIZE]> = Box::new([0; DATA_SIZE]);
	let pc: usize = 0;
	let dc: usize = 0; // Data Pointer

	while pc < code.len()
	{

	}

	Ok(())
}