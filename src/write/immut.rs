
use crate::LockWrite;
use std::io;
use std::fmt;
use std::io::Write;

///Immutability `Trait Write`.
pub trait ImmutWrite<'a> {
	fn write(&'a self, buf: &[u8]) -> io::Result<usize>;
	fn flush(&'a self) -> io::Result<()>;
	fn write_all(&'a self, buf: &[u8]) -> io::Result<()>;
	fn write_fmt(&'a self, fmt: fmt::Arguments) -> io::Result<()>;
}

impl<'a, 'l, E> ImmutWrite<'a> for E where E: LockWrite<'a>, E::LockResult : io::Write {
	fn write(&'a self, buf: &[u8]) -> io::Result<usize> {
		self.lock().write(buf)
	}

	fn flush(&'a self) -> io::Result<()> {
		self.lock().flush()
	}

	fn write_all(&'a self, buf: &[u8]) -> io::Result<()> {
		self.lock().write_all(buf)
	}

	fn write_fmt(&'a self, fmt: fmt::Arguments) -> io::Result<()> {
		self.lock().write_fmt(fmt)
	}
}
