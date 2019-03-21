
use crate::write::ext_write::ExtWrite;

mod guard;
pub use self::guard::*;

use std::sync::MutexGuard;
use std::io::Write;
use std::sync::Mutex;
use std::io;
use std::fmt;

///Unchangeable `Trait Write`.
pub trait ImMutWrite<'a> {
	fn write(&'a self, buf: &[u8]) -> io::Result<usize>;
	fn flush(&'a self) -> io::Result<()>;
	fn write_all(&'a self, buf: &[u8]) -> io::Result<()>;
	fn write_fmt(&'a self, fmt: fmt::Arguments) -> io::Result<()>;
}

impl<'a, E> ImMutWrite<'a> for E where E: ExtWrite<'a> {
	#[inline(always)]
	fn write(&'a self, buf: &[u8]) -> io::Result<usize> {
		self.lock_fn(|mut a| a.write(buf))
	}

	#[inline(always)]
	fn flush(&'a self) -> io::Result<()> {
		self.lock_fn(|mut a| a.flush())
	}

	#[inline(always)]
	fn write_all(&'a self, buf: &[u8]) -> io::Result<()> {
		self.lock_fn(|mut a| a.write_all(buf))
	}

	#[inline(always)]
	fn write_fmt(&'a self, fmt: fmt::Arguments) -> io::Result<()> { 
		self.lock_fn(|mut a| a.write_fmt(fmt))
	}
}





///Combining multiple `Trait Write` into one common.
#[derive(Debug)]
pub struct MutexWrite<T> where T: Write {
	mutex: Mutex<T>
}

impl<T> MutexWrite<T> where T: Write {
	#[inline]
	pub fn new(t: T) -> Self {
		Self::mutex(Mutex::new(t))
	}
	
	#[inline]
	pub const fn mutex(f: Mutex<T>) -> Self {
		MutexWrite {
			mutex: f
		}
	}

	#[inline(always)]
	fn _lock<'a>(&'a self) -> MutexGuard<'a, T> {
		match self.mutex.lock() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		}
	}
}

impl<T> From<T> for MutexWrite<T> where T: Write {
	fn from(a: T) -> Self {
		Self::new(a)
	}
}
impl<T> From<Mutex<T>> for MutexWrite<T> where T: Write {
	fn from(a: Mutex<T>) -> Self {
		Self::mutex(a)
	}
}


impl<T> Write for MutexWrite<T> where T: Write {
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



impl<'a, T: 'a> ExtWrite<'a> for MutexWrite<T> where T: Write {
	type LockWrite = GuardWrite<'a, T>;
	
	#[inline]
	fn lock(&'a self) -> Self::LockWrite {
		Self::LockWrite::from(self._lock())
	}
}
