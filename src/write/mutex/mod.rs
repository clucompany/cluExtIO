
use crate::write::ext_write::ExtWrite;

mod guard;
mod write;
pub use self::guard::*;
pub use self::write::*;

use std::sync::MutexGuard;
use std::io::Write;
use std::sync::Mutex;
use std::io;
use std::fmt;

///Combining multiple `Trait Write` into one common.
#[derive(Debug)]
pub struct MutexWrite<T: Write>(Mutex<T>);

impl<T: Write> MutexWrite<T> {
	#[inline]
	pub fn new(t: T) -> Self {
		Self::mutex(Mutex::new(t))
	}
	#[inline]
	pub fn mutex(m: Mutex<T>) -> Self {
		MutexWrite(m)
	}

	#[inline(always)]
	fn _lock<'a>(&'a self) -> MutexGuard<'a, T> {
		match self.0.lock() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		}
	}
}

impl<T: Write> From<T> for MutexWrite<T> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)
	}
}

impl<T: Write> From<Mutex<T>> for MutexWrite<T> {
	#[inline(always)]
	fn from(a: Mutex<T>) -> Self {
		Self::mutex(a)
	}
}

impl<T: Write> Write for MutexWrite<T> {
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self._lock().write(buf)
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()> {
		self._lock().flush()
	}

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
		self._lock().write_all(buf)
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> { 
		self._lock().write_fmt(fmt)
	}
}

impl<T: Write + Clone> Clone for MutexWrite<T> {
	#[inline]
	fn clone(&self) -> Self {
		Self::new(self._lock().clone())
	}
}



impl<'a, T: 'a + Write> ExtWrite<'a> for MutexWrite<T> {
	type LockWrite = GuardWrite<'a, T>;
	
	#[inline]
	fn lock(&'a self) -> Self::LockWrite {
		GuardWrite::guard(self._lock())
	}
}


impl<T: 'static + Write> Into<Box<Write>> for MutexWrite<T> {
	#[inline]
	fn into(self) -> Box<Write> {
		Box::new(self) as Box<Write>
	}
}
