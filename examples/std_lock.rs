
extern crate cluExtIO;

use std::io::Error;
use cluExtIO::LockWrite;
use std::io::Write;
use std::io;

pub fn main() -> Result<(), Error> {

	let out = std::io::stdout();

	my_function(&out, 0, "No eND:)")?;
	
	out.lock_fn(|mut l| {
		writeln!(l, "End")
	})?;

	Ok( () )
}

fn my_function<'a, W>(raw_write: &'a W, n: usize, str: &'static str) -> Result<(), io::Error> where W: LockWrite<'a>, W::LockResult : io::Write {
	let mut lock = raw_write.lock();

	lock.write_fmt(format_args!("#@{} {}\n", n, "Test"))?;
	lock.write_fmt(format_args!("#@{} {}\n", n+1, "MyString"))?;

	lock.write_fmt(format_args!("#@{} ~{}\n", n+2, str))
}
