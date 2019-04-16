extern crate cluExtIO;

use cluExtIO::ConstUnionWrite;
use std::io::stdout;
use cluExtIO::LockWrite;
use cluExtIO::MutexWrite;
use cluExtIO::drop_write::DropFlush;
use cluExtIO::ImmutWrite;

use std::io::Write;
use std::fs::File;
use std::sync::Arc;
use std::sync::Barrier;
use std::thread;

pub fn main() {
	let arc_out = Arc::new({	  
		let out = stdout();

		let file = DropFlush::new(MutexWrite::new(File::create("/tmp/file.out").unwrap()));
		//Contains the implementation of LockWrite. Safe for inter-thread space.
		//+ Additional self-cleaning after destroying Lock

		let file2 = DropFlush::new(MutexWrite::new(File::create("/tmp/file2.out").unwrap()));
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
				write!(lock, "#@{} {}\n", num_thread, "Thread #OK").unwrap();
				write!(lock, "#@{} {}\n", num_thread, "Thread #T").unwrap();
			});

			barrier.wait();
		});
	}

	barrier.wait();
	
	write!(arc_out, "#@{} {}\n", 999, "Thread pull end.").unwrap();
	//Arc<UnionWrite>, auto lock methods.

	// /tmp/file.out+
	// /tmp/file.out+
}