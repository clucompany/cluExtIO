extern crate cluExtIO;

use cluExtIO::UnionWriteConst;
use cluExtIO::ExtWrite;
use cluExtIO::MutexWrite;
use cluExtIO::FlushLockWrite;
use std::io::Write;
use std::fs::File;

pub fn main() {
     let out = {
          let std_out = std::io::stdout();
          
          let file = FlushLockWrite::new(MutexWrite::new(File::create("/tmp/file.out").unwrap()));
          //Contains the implementation of ExtWrite. Safe for inter-thread space.
          //+ Additional self-cleaning after destroying Lock

          let file2 = FlushLockWrite::new(MutexWrite::new(File::create("/tmp/file2.out").unwrap()));
          //Contains the implementation of ExtWrite. Safe for inter-thread space.
          //+ Additional self-cleaning after destroying Lock
          
          std_out.union(file).union(file2)
     }; //Combined `ExtWrite` with lock function. OUT_PIPE + FILE_PIPE(2) = UNION_SAFE_PIPE

     my_function(&out, 0, "No eND:)").unwrap();
     
     out.lock_fn(|mut l| {
          l.write(b"End.\n")
     }).unwrap();

     // STDOUT+
     // /tmp/file.out+
     // /tmp/file.out+
}

fn my_function<'a, W: ExtWrite<'a>>(raw_write: &'a W, n: usize, str: &'static str) -> Result<(), std::io::Error> {
     let mut lock = raw_write.lock();

     lock.write_fmt(format_args!("#@{} {}\n", n, "Test"))?;
     lock.write_fmt(format_args!("#@{} {}\n", n+1, "MyString"))?;
     lock.write_fmt(format_args!("#@{} ~{}\n", n+2, str))?;

     Ok( () )
}