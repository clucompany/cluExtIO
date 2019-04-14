
mod guard;
use crate::write::generic::WriteStr;
use crate::LockWrite;
pub use self::guard::*;

use std::sync::MutexGuard;
use std::io;
use std::sync::Mutex;
use std::fmt;


///Combining multiple `Trait Write` into one common.
#[derive(Debug)]
pub struct MutexWrite<T> {
	mutex: Mutex<T>
}

impl<T> MutexWrite<T> {
	#[inline]
	pub fn new(t: T) -> Self {
		Mutex::new(t).into()
	}
	
	#[inline]
	pub const fn mutex(f: Mutex<T>) -> Self {
		Self {
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


impl<T> From<T> for MutexWrite<T> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)
	}
}
impl<T> From<Mutex<T>> for MutexWrite<T> {
	#[inline(always)]
	fn from(a: Mutex<T>) -> Self {
		Self::mutex(a)
	}
}


impl<T> io::Write for MutexWrite<T> where T: io::Write {
	fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
		self._lock().write(buf)
	}

	fn flush(&mut self) -> Result<(), io::Error> {
		self._lock().flush()
	}

	fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
		self._lock().write_all(buf)
	}

	fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), io::Error> { 
		self._lock().write_fmt(fmt)
	}
}

/*
impl<'a, T> ImmutWrite<'a> for MutexWrite<T> where T: ImmutWrite<'a> {
	fn write(&'a self, buf: &[u8]) -> Result<usize, io::Error> {
		self._lock().write(buf)
	}

	fn flush(&'a self) -> Result<(), io::Error> {
		self._lock().flush()
	}

	fn write_all(&'a self, buf: &[u8]) -> Result<(), io::Error> {
		self._lock().write_all(buf)
	}

	fn write_fmt(&'a self, fmt: fmt::Arguments) -> Result<(), io::Error> { 
		self._lock().write_fmt(fmt)
	}
}*/


impl<T> fmt::Write for MutexWrite<T> where T: fmt::Write {
	fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
		self._lock().write_str(s)
	}
	
	fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
		self._lock().write_char(c)
	}
	
	fn write_fmt(self: &mut Self, args: fmt::Arguments) -> Result<(), fmt::Error> {
		self._lock().write_fmt(args)
	}
}

impl<T, OK, ERR> WriteStr for MutexWrite<T> where T: WriteStr<Ok = OK, Err = ERR> {
	type Ok = OK;
	type Err = ERR;
	
	
	fn write_str(&mut self, s: &str) -> Result<Self::Ok, Self::Err> {
		self._lock().write_str(s)
	}
	
	fn write_str_array<'l>(&mut self, arr: &'l [&str]) -> Result<(), Self::Err> {
		self._lock().write_str_array(arr)
	}
	
	fn write_str_lenarray<'l>(&mut self, all_size: usize, arr: &'l [&str]) -> Result<(), Self::Err> {
		self._lock().write_str_lenarray(all_size, arr)
	}
}




impl<'a, T:'a > LockWrite<'a> for MutexWrite<T>  {
	type LockResult = GuardWrite<'a, T>;
	
	fn lock(&'a self) -> Self::LockResult {
		self._lock().into()
	}
}
