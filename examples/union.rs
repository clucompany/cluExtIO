

extern crate cluExtIO;

use std::io::Write;
use cluExtIO::UnionWriteConst;
use std::fs::File;

pub fn main() {

     let file1 = File::create("/tmp/1.out").unwrap();
     //file1 - `Write trait`

     let file2 = File::create("/tmp/2.out").unwrap();
     //file2 - `Write trait`

     let write = file1.union(file2);
     //UnionWrite - `FILE1+FILE2`


     my_function(write).unwrap();
}

fn my_function<W: Write>(mut w: W) -> Result<(), std::io::Error> {
     w.write_fmt(format_args!("#@{} {}\n", 1, "Test"))?;
     w.write_fmt(format_args!("#@{} {}\n", 2, "MyString"))?;
     w.write_fmt(format_args!("#@{} {}\n", 3, "MyString"))?;

     Ok( () )
}

