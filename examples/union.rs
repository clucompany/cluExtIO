

extern crate cluExtIO;

use std::io::Error;
use std::io::Write;
use cluExtIO::UnionWriteConst;
use std::fs::File;

pub fn main() -> Result<(), Error> {

     let file1 = File::create("/tmp/1.out")?;
     //file1 - `Write trait`

     let file2 = File::create("/tmp/2.out")?;
     //file2 - `Write trait`

     let write = file1.union(file2);
     //UnionWrite - `FILE1+FILE2`


     my_function(write)
}

fn my_function<W: Write>(mut w: W) -> Result<(), Error> {
     w.write_fmt(format_args!("#@{} {}\n", 1, "Test"))?;
     w.write_fmt(format_args!("#@{} {}\n", 2, "MyString"))?;
     
     w.write_fmt(format_args!("#@{} {}\n", 3, "MyString"))
}

