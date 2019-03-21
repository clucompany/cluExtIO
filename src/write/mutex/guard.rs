
use std::ops::DerefMut;
use std::ops::Deref;
use std::sync::MutexGuard;
use std::io::Write;
use std::io;
use std::fmt;

#[derive(Debug)]
pub struct GuardWrite<'a, T> where T: Write { 
	guard: MutexGuard<'a, T>
}

impl<'a, T> GuardWrite<'a, T> where T: Write {
	#[inline]
	pub const fn guard(t: MutexGuard<'a, T>) -> Self {
		GuardWrite {
			guard: t	
		}
	}
}

impl<'a, T> Deref for GuardWrite<'a, T> where T: Write {
	type Target = MutexGuard<'a, T>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.guard	
	}
}

impl<'a, T> DerefMut for GuardWrite<'a, T> where T: Write {	
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.guard
	}
}


impl<'a, T> From<MutexGuard<'a, T>> for GuardWrite<'a, T> where T: Write {
	#[inline(always)]
	fn from(a: MutexGuard<'a, T>) -> Self {
		Self::guard(a)
	}
}


impl<'a, T> Write for GuardWrite<'a, T> where T: Write {
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.guard.write(buf)
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()> {
		self.guard.flush()
	}

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
		self.guard.write_all(buf)
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> { 
		self.guard.write_fmt(fmt)
	}
}
