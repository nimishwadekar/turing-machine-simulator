use std::{io::Write, collections::HashMap};

use console::Term;

fn parse_brackets(code: &Vec<u8>) -> Result<HashMap<usize, usize>, String>
{
	let mut jmp_map: HashMap<usize, usize> = HashMap::new();
	let mut stack: Vec<usize> = Vec::new();
	let mut pc = 0;

	while pc < code.len()
	{
		match code[pc] as char
		{
			'[' => { stack.push(pc); },
			']' => 
			{ 
				match stack.pop()
				{
					Some(val) => { jmp_map.insert(val, pc); jmp_map.insert(pc, val); }
					None => { return Err(String::from("Mismatched brackets")); }
				} 
			},

			_ => {},
		};

		pc += 1;
	}

	if stack.len() > 0
	{
		Err(String::from("Mismatched brackets"))
	}
	else
	{
		Ok(jmp_map)
	}
}

pub fn interpret(code: Vec<u8>) -> Result<(), String>
{
	const DATA_SIZE: usize = 65536;

	let mut data: Box<[u8; DATA_SIZE]> = Box::new([0; DATA_SIZE]);
	let mut pc = 0;
	let mut dc = 0; // Data Pointer

	let mut term = Term::stdout();

	let mut buf: [u8; 1] = [0; 1];

	let jmp_map = parse_brackets(&code)?;

	while pc < code.len()
	{
		match code[pc] as char
		{
			'>' => { if dc < DATA_SIZE - 1 { dc += 1; } },
			'<' => { if dc > 0 { dc -= 1; } },
			'+' => { data[dc] += 1 },
			'-' => { data[dc] -= 1 },
			'.' => { buf[0] = data[dc]; term.write(&buf).expect("Display failed"); },
			',' => { data[dc] = term.read_char().expect("Read failed") as u8; },
			'[' => { pc = *jmp_map.get(&pc).unwrap() - 1; },
			']' => { if data[dc] != 0 { pc = *jmp_map.get(&pc).unwrap(); } },

			'\n' => {},
			' ' => {},
			'\r' => {},

			sym => { return Err(format!("Unrecognized symbol '{}'", sym)); },
		};

		pc += 1;
	}

	Ok(())
}