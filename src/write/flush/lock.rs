
use std::ops::DerefMut;
use std::ops::Deref;
use crate::write::flush::FlushDropWrite;
use crate::write::ext_write::ExtWrite;

use std::marker::PhantomData;
use std::io::Write;
use std::io;
use std::fmt;

///An implementation of `Trait Write` that calls the flush() method when removing a lock.
#[derive(Debug)]
pub struct FlushLockWrite<'a, T> where T: ExtWrite<'a> {
	write: T, 
	_phantom: PhantomData<&'a ()>,
}

impl<'a, T> FlushLockWrite<'a, T> where T: ExtWrite<'a> {
	#[inline]
	pub const fn new(a: T) -> Self {
		Self {
			write: a,
			_phantom: PhantomData,	
		}
	}
}

impl<'a, T> Deref for FlushLockWrite<'a, T> where T: ExtWrite<'a> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.write	
	}	
}
impl<'a, T> DerefMut for FlushLockWrite<'a, T> where T: ExtWrite<'a> {	
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.write
	}	
}

impl<'a, T> From<T> for FlushLockWrite<'a, T> where T: ExtWrite<'a> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)
	}
}


impl<'a, T> Write for FlushLockWrite<'a, T> where T: ExtWrite<'a> {
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.write.write(buf)
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()> {
		self.write.flush()
	}

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
		self.write.write_all(buf)
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> {
		self.write.write_fmt(fmt)
	}
}

impl<'a, T> ExtWrite<'a> for FlushLockWrite<'a, T> where T: ExtWrite<'a> {
	type LockWrite = FlushDropWrite<T::LockWrite>;

	#[inline]
	fn lock(&'a self) -> Self::LockWrite {
		self.write.lock().into()
	}
}

impl<'a, T> Clone for FlushLockWrite<'a, T> where T: ExtWrite<'a> + Clone {
	#[inline]
	fn clone(&self) -> Self {
		Self::new(self.write.clone())
	}
}
