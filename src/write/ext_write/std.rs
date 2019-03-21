
use crate::write::ext_write::ExtWrite;

use std::io::StderrLock;
use std::io::Stderr;
use std::io::StdoutLock;
use std::io::Stdout;

impl<'a> ExtWrite<'a> for Stdout {	
	type LockWrite = StdoutLock<'a>;

	#[inline(always)]
	fn lock(&'a self) -> Self::LockWrite {
		self.lock()
	}
}

impl<'a> ExtWrite<'a> for Stderr {
	type LockWrite = StderrLock<'a>;

	#[inline(always)]
	fn lock(&'a self) -> Self::LockWrite {
		self.lock()
	}
}

