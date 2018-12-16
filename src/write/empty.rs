
use crate::write::ext_write::ExtWrite;

use std::io;
use std::io::Write;
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
}

impl From<()> for EmptyWrite {
	#[inline(always)]
	fn from(_a: ()) -> Self {
		Self::new()
	}
}


impl Write for EmptyWrite {
	#[inline(always)]
	fn write<'a>(&mut self, _buf: &'a [u8]) -> io::Result<usize> {
		Ok( 0 )
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()> {
		Ok( () )
	}

     #[inline(always)]
     fn write_all<'a>(&mut self, _buf: &'a [u8]) -> io::Result<()> {
          Ok( () )
     }

     #[inline(always)]
     fn write_fmt(&mut self, _fmt: fmt::Arguments) -> io::Result<()> { 
          Ok( () )
     }
}

impl Clone for EmptyWrite {
     #[inline(always)]
     fn clone(&self) -> Self {
          EmptyWrite
     }
}

impl<'a> ExtWrite<'a> for EmptyWrite {    
     type LockWrite = GuardEmptyWrite; 

     #[inline]
     fn lock(&self) -> Self::LockWrite {
          GuardEmptyWrite::new()
     }
}


impl Into<Box<Write>> for EmptyWrite {
     #[inline]
     fn into(self) -> Box<Write> {
          Box::new(self) as Box<Write>
     }
}
