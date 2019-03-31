
use crate::lock_write::LockWrite;
use std::io;
use std::fmt;

pub type GuardEmptyWrite = EmptyWrite;

///An implementation of `Trait Write` that does nothing.
#[derive(Debug)]
pub struct EmptyWrite;

impl EmptyWrite {
	#[inline(always)]
	pub const fn new() -> Self {
		EmptyWrite
	}
}

impl From<()> for EmptyWrite {
	#[inline(always)]
	fn from(_a: ()) -> Self {
		Self::new()
	}
}


impl io::Write for EmptyWrite {
	#[inline(always)]
	fn write(&mut self, _buf: &[u8]) -> Result<usize, io::Error> {
		Ok( 0 )
	}

	#[inline(always)]
	fn flush(&mut self) -> Result<(), io::Error> {
		Ok( () )
	}

	#[inline(always)]
	fn write_all(&mut self, _buf: &[u8]) -> Result<(), io::Error> {
		Ok( () )
	}

	#[inline(always)]
	fn write_fmt(&mut self, _fmt: fmt::Arguments) -> Result<(), io::Error> {
		Ok( () )
	}
}

impl fmt::Write for EmptyWrite {
	#[inline(always)]
	fn write_str(&mut self, _s: &str) -> Result<(), fmt::Error> {
		Ok( () )
	}
	
	#[inline(always)]
	fn write_char(&mut self, _c: char) -> Result<(), fmt::Error> {
		Ok( () )
	}
	
	#[inline(always)]
	fn write_fmt(&mut self, _args: fmt::Arguments) -> Result<(), fmt::Error> {
		Ok( () )
	}
}


impl Clone for EmptyWrite {
	#[inline(always)]
	fn clone(&self) -> Self {
		EmptyWrite
	}
}

impl<'a> LockWrite<'a> for EmptyWrite {	
	type LockResult = GuardEmptyWrite; 

	#[inline]
	fn lock(&self) -> Self::LockResult {
		Self::LockResult::new()
	}
}
