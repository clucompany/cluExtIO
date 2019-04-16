

extern crate cluExtIO;

use cluExtIO::ConstUnionWrite;
use std::io;
use std::io::Write;
use std::fs::File;

pub fn main() -> Result<(), io::Error> {

	let file1 = File::create("/tmp/1.out")?;
	//file1 - `Write trait`

	let file2 = File::create("/tmp/2.out")?;
	//file2 - `Write trait`

	let write = file1.union(file2);
	//UnionWrite - `FILE1+FILE2`


	my_function(write)
}

fn my_function<W: Write>(mut w: W) -> Result<(), io::Error> {
	write!(w, "#@{} {}\n", 1, "Test")?;
	write!(w, "#@{} {}\n", 2, "MyString")?;
	
	write!(w, "#@{} {}\n", 3, "MyString")
}

