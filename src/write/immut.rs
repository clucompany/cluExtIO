
use crate::LockWrite;
use std::io;
use std::fmt;
use std::io::Write;

///Immutability `Trait Write`.
pub trait ImmutWrite<'a> {
	type Err;
	
	fn write<'l>(&'a self, buf: &'l [u8]) -> Result<usize, Self::Err>;
	
	fn flush(&'a self) -> Result<(), Self::Err>;
	fn write_all<'l>(&'a self, buf: &'l [u8]) -> Result<(), Self::Err>;
	fn write_fmt<'l>(&'a self, fmt: fmt::Arguments<'l>) -> Result<(), Self::Err>;
}

impl<'a, 't, E: 't> ImmutWrite<'a> for E where E: LockWrite<'a>, E::LockResult : io::Write {
	type Err = io::Error;
	
	fn write<'l>(&'a self, buf: &'l [u8]) -> Result<usize, Self::Err> {
		self.lock().write(buf)
	}

	fn flush(&'a self) -> Result<(), Self::Err> {
		self.lock().flush()
	}

	fn write_all<'l>(&'a self, buf: &'l [u8]) -> Result<(), Self::Err> {
		self.lock().write_all(buf)
	}

	fn write_fmt<'l>(&'a self, fmt: fmt::Arguments<'l>) -> Result<(), Self::Err> {
		self.lock().write_fmt(fmt)
	}
}

