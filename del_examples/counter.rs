
extern crate cluExtIO;

use std::io::Write;
use cluExtIO::analytics::CounterIOWrite;
use std::io;

fn main() -> Result<(), io::Error> {
	let out = io::stdout();
	let mut counter = CounterIOWrite::from(out.lock());
	
	for _a in 0..3 {
		counter.write(b"11")?;
	}
	counter.flush()?;
	
	println!();
	println!("#Debug out:");
	counter.write_stat(&mut out.lock())?;
	
	Ok( () )
}
