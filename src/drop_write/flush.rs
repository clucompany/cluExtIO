
use crate::LockWrite;
use crate::write::generic::FlushWrite;
use std::ops::DerefMut;
use std::ops::Deref;
use std::marker::PhantomData;
use std::io;
use std::fmt;

#[derive(Debug)]
pub struct DropFlushWrite<T, E> where T: FlushWrite<E> {
	write: T,
	_p: PhantomData<E>,
}

impl<T, E> DropFlushWrite<T, E> where T: FlushWrite<E> {
	#[inline]
	pub const fn new(write: T) -> Self {
		Self {
			write: write,
			_p: PhantomData,
		}
	}
}

impl<T, E> From<T> for DropFlushWrite<T, E> where T: FlushWrite<E> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)
	}
}

impl<T, E> Deref for DropFlushWrite<T, E> where T: FlushWrite<E> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &T {
		&self.write
	}
}

impl<T, E> DerefMut for DropFlushWrite<T, E> where T: FlushWrite<E> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.write
	}
}


impl<T, E> io::Write for DropFlushWrite<T, E> where T: FlushWrite<E> + io::Write {
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
		self.write.write(buf)	
	}
	
	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
		self.write.write_all(buf)
	}
	
	#[inline(always)]
	fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), io::Error> {
		self.write.write_fmt(fmt)
	}
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), io::Error> {
		io::Write::flush(&mut self.write)
	}
}


impl<'a, T> LockWrite<'a> for DropFlushWrite<T, io::Error> where T: LockWrite<'a> + FlushWrite<io::Error> + io::Write {
	type LockResult = T::LockResult;
	
	#[inline(always)]
	fn lock(&'a self) -> Self::LockResult {
		self.write.lock()
	}
}



impl<T, E> Drop for DropFlushWrite<T, E> where T: FlushWrite<E> {
	fn drop(&mut self) {
		let _e = self.write.flush();
	}
}
