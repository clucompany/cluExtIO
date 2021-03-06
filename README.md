# cluExtIO

[![Build Status](https://travis-ci.org/clucompany/cluExtIO.svg?branch=master)](https://travis-ci.org/clucompany/cluExtIO)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluExtIO)](https://crates.io/crates/cluExtIO)
[![Documentation](https://docs.rs/cluExtIO/badge.svg)](https://docs.rs/cluExtIO)

Syntactic sugar extends I/O capabilities.


# Capabilities:
1. EmptyWrite - Empty "Write" that does nothing.
2. UnionWrite - Possibility to combine several "Write" into one record.
3. MutexWrite - Combining Mutex and Write for multi-threaded access.
4. LockWrite - The trait extends the capabilities of the standard Write, adds lock methods.
5. FlushDropWrite - An implementation of "Trait Write", which calls the flush () method on drop. 
6. FlushLockWrite - An implementation of "Trait Write" that calls the flush() method when removing a lock.
7. NotChanWrite - Unchangeable "Trait Write".
...

# Use

1. UnionWrite

```rust
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
```
		
2. LockWrite

```rust
extern crate cluExtIO;

use cluExtIO::LockWrite;
use std::io::Write;

pub fn main() {
	let out = std::io::stdout();

	my_function(&out, 0, "No eND:)").unwrap();

	out.lock_fn(|mut l| {
			l.write(b"End.\n")
	}).unwrap();
}

fn my_function<'a, W: LockWrite<'a>>(raw_write: &'a W, n: usize, str: &'static str) -> Result<(), std::io::Error> {
	let mut lock = raw_write.lock();

	lock.write_fmt(format_args!("#@{} {}\n", n, "Test"))?;
	lock.write_fmt(format_args!("#@{} {}\n", n+1, "MyString"))?;
	lock.write_fmt(format_args!("#@{} ~{}\n", n+2, str))?;

	Ok( () )
}
```

3. Threads

```rust
extern crate cluExtIO;

use std::io::stdout;
use cluExtIO::UnionWriteConst;
use cluExtIO::LockWrite;
use cluExtIO::MutexWrite;
use cluExtIO::FlushLockWrite;
use cluExtIO::NotChanWrite;
use std::io::Write;
use std::fs::File;
use std::sync::Arc;
use std::sync::Barrier;
use std::thread;

pub fn main() {
		 let arc_out = Arc::new({	  
				let out = stdout();

				let file = FlushLockWrite::new(MutexWrite::new(File::create("/tmp/file.out").unwrap()));
				//Contains the implementation of LockWrite. Safe for inter-thread space.
				//+ Additional self-cleaning after destroying Lock

				let file2 = FlushLockWrite::new(MutexWrite::new(File::create("/tmp/file2.out").unwrap()));
				//Contains the implementation of LockWrite. Safe for inter-thread space.
				//+ Additional self-cleaning after destroying Lock

				out.union(file).union(file2)
		 }); //Combined `LockWrite` with lock function. OUT_PIPE + FILE_PIPE(2) = UNION_SAFE_PIPE


		 let barrier = Arc::new(Barrier::new(5 + 1));

		 for num_thread in 0..5 {
				let barrier = barrier.clone();
				let arc_out = arc_out.clone();
				thread::spawn(move || {

						arc_out.lock_fn(|mut lock| {
								lock.write_fmt(format_args!("#@{} {}\n", num_thread, "Thread #OK")).unwrap();
								lock.write_fmt(format_args!("#@{} {}\n", num_thread, "Thread #T")).unwrap();
						});

						barrier.wait();
				});
		 }

		 barrier.wait();

		 arc_out.write_fmt(format_args!("#@{} {}\n", 999, "Thread pull end.")).unwrap();
		 //Arc<UnionWrite>, auto lock methods. (NotChanWrite)

		 // /tmp/file.out+
		 // /tmp/file.out+
}
```


# License

Copyright 2018 #UlinProject Денис Котляров

Licensed under the Apache License, Version 2.0
