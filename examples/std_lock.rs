
extern crate cluExtIO;

use std::io::Error;
use cluExtIO::ExtWrite;
use std::io::Write;

pub fn main() -> Result<(), Error> {

	let out = std::io::stdout();

	my_function(&out, 0, "No eND:)")?;
	
	out.lock_fn(|mut l| {
		l.write(b"End.\n")
	})?;

	Ok( () )
}

fn my_function<'a, W: ExtWrite<'a>>(raw_write: &'a W, n: usize, str: &'static str) -> Result<(), std::io::Error> {
	let mut lock = raw_write.lock();

	lock.write_fmt(format_args!("#@{} {}\n", n, "Test"))?;
	lock.write_fmt(format_args!("#@{} {}\n", n+1, "MyString"))?;

	lock.write_fmt(format_args!("#@{} ~{}\n", n+2, str))
}