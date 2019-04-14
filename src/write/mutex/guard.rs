
use crate::write::generic::WriteStr;
use crate::write::lock::LockWrite;
use std::ops::DerefMut;
use std::ops::Deref;
use std::sync::MutexGuard;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct GuardWrite<'a, T>(pub MutexGuard<'a, T>);

impl<'a, T> GuardWrite<'a, T> {
	#[inline]
	pub const fn guard(t: MutexGuard<'a, T>) -> Self {
		GuardWrite(t)
	}
	
	#[inline(always)]
	pub const fn as_ref(&self) -> &MutexGuard<'a, T> {
		&self.0
	}
}

impl<'a, T> Deref for GuardWrite<'a, T> {
	type Target = MutexGuard<'a, T>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a, T> DerefMut for GuardWrite<'a, T> {	
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}


impl<'a, T> From<MutexGuard<'a, T>> for GuardWrite<'a, T> {
	#[inline(always)]
	fn from(a: MutexGuard<'a, T>) -> Self {
		Self::guard(a)
	}
}


impl<'a, T> io::Write for GuardWrite<'a, T> where T: io::Write {
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
		self.0.write(buf)
	}

	#[inline(always)]
	fn flush(&mut self) -> Result<(), io::Error> {
		self.0.flush()
	}

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
		self.0.write_all(buf)
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), io::Error> { 
		self.0.write_fmt(fmt)
	}
}


/*impl<'a, T> ImmutWrite<'a> for GuardWrite<'a, T> where T: ImmutWrite<'a> {
	fn write(&'a self, buf: &[u8]) -> Result<usize, io::Error> {
		self.0.write(buf)
	}

	fn flush(&'a self) -> Result<(), io::Error> {
		self.0.flush()
	}

	fn write_all(&'a self, buf: &[u8]) -> Result<(), io::Error> {
		self.0.write_all(buf)
	}

	fn write_fmt(&'a self, fmt: fmt::Arguments) -> Result<(), io::Error> { 
		self.0.write_fmt(fmt)
	}
}*/

impl<'a, T, OK, ERR> WriteStr for GuardWrite<'a, T> where T: WriteStr<Ok = OK, Err = ERR> {
	type Ok = OK;
	type Err = ERR;
	
	
	fn write_str(&mut self, s: &str) -> Result<Self::Ok, Self::Err> {
		self.0.write_str(s)
	}
	
	fn write_str_array<'l>(&mut self, arr: &'l [&str]) -> Result<(), Self::Err> {
		self.0.write_str_array(arr)
	}
	
	fn write_str_lenarray<'l>(&mut self, all_size: usize, arr: &'l [&str]) -> Result<(), Self::Err> {
		self.0.write_str_lenarray(all_size, arr)
	}
}

impl<'a, T> fmt::Write for GuardWrite<'a, T> where T: fmt::Write {
	fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
		self.0.write_str(s)
	}
	
	fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
		self.0.write_char(c)
	}
	
	fn write_fmt(self: &mut Self, args: fmt::Arguments) -> Result<(), fmt::Error> {
		self.0.write_fmt(args)
	}
}


impl<'a, T> LockWrite<'a> for GuardWrite<'a, T> {
	type LockResult = &'a GuardWrite<'a, T>;
	
	#[inline(always)]
	fn lock(&'a self) -> Self::LockResult {
		self
	}
}
