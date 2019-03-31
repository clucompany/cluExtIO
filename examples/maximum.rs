extern crate cluExtIO;

use cluExtIO::ConstUnionWrite;
use std::io::Error;
use cluExtIO::LockWrite;
use cluExtIO::MutexWrite;
use cluExtIO::drop_write::DropWriteFlush;
use std::io::Write;
use std::io;
use std::fs::File;

pub fn main() -> Result<(), Error> {
	let out = {
		let std_out = std::io::stdout();
		
		let file = DropWriteFlush::from(MutexWrite::from(File::create("/tmp/file.out")?));
		//Contains the implementation of LockWrite. Safe for inter-thread space.
		//+ Additional self-cleaning after destroying Lock

		let file2 = DropWriteFlush::from(MutexWrite::from(File::create("/tmp/file2.out")?));
		//Contains the implementation of LockWrite. Safe for inter-thread space.
		//+ Additional self-cleaning after destroying Lock
		
		std_out.union(file).union(file2)
	}; //Combined `LockWrite` with lock function. OUT_PIPE + FILE_PIPE(2) = UNION_SAFE_PIPE

	my_function(&out, 0, "No eND:)")?;
	
	out.lock_fn(|mut l| {
		l.write(b"End.\n")
	})?;

	// STDOUT+
	// /tmp/file.out+
	// /tmp/file.out+

	Ok( () )
}

fn my_function<'a, W>(raw_write: &'a W, n: usize, str: &'static str) -> Result<(), io::Error> where W: LockWrite<'a>, W::LockResult : io::Write {
	let mut lock = raw_write.lock();

	lock.write_fmt(format_args!("#@{} {}\n", n, "Test"))?;
	lock.write_fmt(format_args!("#@{} {}\n", n+1, "MyString"))?;
	
	lock.write_fmt(format_args!("#@{} ~{}\n", n+2, str))
}
