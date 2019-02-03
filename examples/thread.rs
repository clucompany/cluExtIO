extern crate cluExtIO;

use std::io::stdout;
use cluExtIO::UnionWriteConst;
use cluExtIO::ExtWrite;
use cluExtIO::MutexWrite;
use cluExtIO::FlushLockWrite;
use cluExtIO::ImMutWrite;

use std::io::Write;
use std::fs::File;
use std::sync::Arc;
use std::sync::Barrier;
use std::thread;

pub fn main() {
	let arc_out = Arc::new({	  
		let out = stdout();

		let file = FlushLockWrite::new(MutexWrite::new(File::create("/tmp/file.out").unwrap()));
		//Contains the implementation of ExtWrite. Safe for inter-thread space.
		//+ Additional self-cleaning after destroying Lock

		let file2 = FlushLockWrite::new(MutexWrite::new(File::create("/tmp/file2.out").unwrap()));
		//Contains the implementation of ExtWrite. Safe for inter-thread space.
		//+ Additional self-cleaning after destroying Lock
		
		out.union(file).union(file2)
	}); //Combined `ExtWrite` with lock function. OUT_PIPE + FILE_PIPE(2) = UNION_SAFE_PIPE


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
	//Arc<UnionWrite>, auto lock methods.

	// /tmp/file.out+
	// /tmp/file.out+
}