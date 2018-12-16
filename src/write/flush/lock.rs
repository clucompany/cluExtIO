
use crate::write::flush::FlushDropWrite;
use crate::write::ext_write::ExtWrite;

use std::marker::PhantomData;
use std::io::Write;
use std::io;
use std::fmt;

///An implementation of `Trait Write` that calls the flush() method when removing a lock.
#[derive(Debug)]
pub struct FlushLockWrite<'a, T: ExtWrite<'a>>(T, PhantomData<&'a ()>);

impl<'a, T: ExtWrite<'a>> FlushLockWrite<'a, T> {
	#[inline]
	pub fn new(a: T) -> Self {
		FlushLockWrite(a, PhantomData)
	}
}

impl<'a, T: ExtWrite<'a>> From<T> for FlushLockWrite<'a, T> {
	#[inline]
	fn from(a: T) -> Self {
		FlushLockWrite::new(a)
	}
}


impl<'a, T: ExtWrite<'a>> Write for FlushLockWrite<'a, T> {
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.0.write(buf)
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()> {
		self.0.flush()
	}

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
		self.0.write_all(buf)
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> {
		self.0.write_fmt(fmt)
	}
}

impl<'a, T: ExtWrite<'a>> ExtWrite<'a> for FlushLockWrite<'a, T> {
     type LockWrite = FlushDropWrite<T::LockWrite>;

     #[inline]
     fn lock(&'a self) -> Self::LockWrite {
          FlushDropWrite::new(self.0.lock())
     }
}

impl<'a, T: ExtWrite<'a> + Clone> Clone for FlushLockWrite<'a, T> {
     #[inline]
     fn clone(&self) -> Self {
          Self::new(self.0.clone())
     }
}