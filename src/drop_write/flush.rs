
use crate::LockWrite;
use crate::write::generic::WriteFlush;
use std::ops::DerefMut;
use std::ops::Deref;
use std::marker::PhantomData;
use std::io;
use std::fmt;

#[derive(Debug)]
pub struct DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	write: T,
	_p: PhantomData<E>,
}

impl<T, E> DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	#[inline]
	pub const fn new(write: T) -> Self {
		Self {
			write: write,
			_p: PhantomData,
		}
	}
}

impl<T, E> From<T> for DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)
	}
}

impl<T, E> Deref for DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &T {
		&self.write
	}
}

impl<T, E> DerefMut for DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.write
	}
}


impl<T, E> io::Write for DropWriteFlush<T, E> where T: WriteFlush<Err = E> + io::Write {
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


impl<'a, T> LockWrite<'a> for DropWriteFlush<T, io::Error> where T: LockWrite<'a> + WriteFlush<Err = io::Error> + io::Write {
	type LockResult = T::LockResult;
	
	#[inline(always)]
	fn lock(&'a self) -> Self::LockResult {
		self.write.lock()
	}
}



impl<T, E> Drop for DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	fn drop(&mut self) {
		let _e = self.write.flush();
	}
}
