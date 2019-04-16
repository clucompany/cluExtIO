
extern crate cluExtIO;

use std::io::Write;
use std::io;
use cluExtIO::drop_write::DropFlush;
use cluExtIO::FlushFn;

fn main() -> Result<(), io::Error> {
	let mut vec = Vec::<u8>::with_capacity(24);
	
	{
		let mut write_buff = DropFlush::new(FlushFn::new(&mut vec, |vec| {
			println!("Flush {:?}", vec);
		}));
		
		write_buff.write(b"0")?;
		write_buff.flush()?;
		write_buff.write(b"1")?;
		
	}
	
	let mut stdout = io::stdout();
	stdout.write(vec.as_slice())?;
	stdout.write(b"\n")?;
	
	Ok( () )
}