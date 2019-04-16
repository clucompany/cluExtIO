
use crate::lock_write::LockWrite;
use std::io;
use std::fmt;

pub type GuardEmptyWrite = EmptyWrite;

///An implementation of `Trait Write` that does nothing.
#[derive(Debug)]
pub struct EmptyWrite;

impl EmptyWrite {
	#[inline]
	pub const fn new() -> Self {
		EmptyWrite
	}
	
	#[inline]
	pub fn as_write(&mut self) -> &mut dyn io::Write {
		self
	}
	
	#[inline]
	pub fn as_inwrite(&mut self) -> &mut dyn ImmutWrite {
		self
	}
}

impl From<()> for EmptyWrite {
	#[inline(always)]
	fn from(_a: ()) -> Self {
		Self::new()
	}
}


impl io::Write for EmptyWrite {
	#[inline]
	fn write(&mut self, _buf: &[u8]) -> Result<usize, io::Error> {
		Ok( 0 )
	}

	#[inline]
	fn flush(&mut self) -> Result<(), io::Error> {
		Ok( () )
	}

	#[inline]
	fn write_all(&mut self, _buf: &[u8]) -> Result<(), io::Error> {
		Ok( () )
	}

	#[inline]
	fn write_fmt(&mut self, _fmt: fmt::Arguments) -> Result<(), io::Error> {
		Ok( () )
	}
}

impl io::Write for () {
	#[inline]
	fn write(&mut self, _buf: &[u8]) -> Result<usize, io::Error> {
		Ok( 0 )
	}

	#[inline]
	fn flush(&mut self) -> Result<(), io::Error> {
		Ok( () )
	}

	#[inline]
	fn write_all(&mut self, _buf: &[u8]) -> Result<(), io::Error> {
		Ok( () )
	}

	#[inline]
	fn write_fmt(&mut self, _fmt: fmt::Arguments) -> Result<(), io::Error> {
		Ok( () )
	}
}



impl fmt::Write for EmptyWrite {
	#[inline]
	fn write_str(&mut self, _s: &str) -> Result<(), fmt::Error> {
		Ok( () )
	}
	
	#[inline]
	fn write_char(&mut self, _c: char) -> Result<(), fmt::Error> {
		Ok( () )
	}
	
	#[inline]
	fn write_fmt(&mut self, _args: fmt::Arguments) -> Result<(), fmt::Error> {
		Ok( () )
	}
}


impl fmt::Write for () {
	#[inline]
	fn write_str(&mut self, _s: &str) -> Result<(), fmt::Error> {
		Ok( () )
	}
	
	#[inline]
	fn write_char(&mut self, _c: char) -> Result<(), fmt::Error> {
		Ok( () )
	}
	
	#[inline]
	fn write_fmt(&mut self, _args: fmt::Arguments) -> Result<(), fmt::Error> {
		Ok( () )
	}
}


impl<'a> ImmutWrite<'a> for EmptyWrite {
	#[inline]
	fn write(&self, _buf: &[u8]) -> Result<usize, io::Error> {
		Ok( 0 )
	}

	#[inline]
	fn flush(&self) -> Result<(), io::Error> {
		Ok( () )
	}

	#[inline]
	fn write_all(&self, _buf: &[u8]) -> Result<(), io::Error> {
		Ok( () )
	}

	#[inline]
	fn write_fmt(&self, _fmt: fmt::Arguments) -> Result<(), io::Error> {
		Ok( () )
	}
}



impl Clone for EmptyWrite {
	#[inline(always)]
	fn clone(&self) -> Self {
		EmptyWrite::new()
	}
}

impl<'a> LockWrite<'a> for EmptyWrite {
	type LockResult = GuardEmptyWrite; 

	#[inline]
	fn lock(&self) -> Self::LockResult {
		Self::LockResult::new()
	}
}
