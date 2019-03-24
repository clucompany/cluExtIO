
use std::ops::DerefMut;
use std::ops::Deref;
use std::io::Write;
use std::io;
use std::fmt;

///An implementation of `Trait Write`, which calls the flush() method on drop.
#[derive(Debug)]							
pub struct FlushDropWrite<T> where T: Write {
	write: T
}

impl<T> FlushDropWrite<T> where T: Write {
	#[inline]
	pub const fn new(a: T) -> Self {
		FlushDropWrite {
			write: a	
		}
	}

	#[inline(always)]
	pub fn flush(self) {}
}

impl<T> Deref for FlushDropWrite<T> where T: Write {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.write	
	}
}

impl<T> DerefMut for FlushDropWrite<T> where T: Write {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.write
	}
}


impl<T> From<T> for FlushDropWrite<T> where T: Write {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)
	}
}


impl<T> Write for FlushDropWrite<T> where T: Write {
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

impl<T> Clone for FlushDropWrite<T> where T: Write + Clone {
	#[inline]
	fn clone(&self) -> Self {
		Self::new(self.write.clone())
	}
}

impl<T> Drop for FlushDropWrite<T> where T: Write {
	#[inline(always)]
	fn drop(&mut self) {
		let _e = self.write.flush();
	}
}
