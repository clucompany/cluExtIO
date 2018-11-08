
use std::io::Write;

#[derive(Debug)]
///An implementation of "Trait Write", which calls the flush () method on drop.                                  
pub struct FlushWrite<T: Write>(T);

impl<T: Write> FlushWrite<T> {
	#[inline]
	pub fn new(a: T) -> Self {
		FlushWrite(a)
	}

	#[inline(always)]
	pub fn flush(self) {}
}

impl<T: Write> Write for FlushWrite<T> {
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
		self.0.write(buf)
	}

	#[inline(always)]
	fn flush(&mut self) -> ::std::io::Result<()> {
		self.0.flush()
	}

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()> {
		self.0.write_all(buf)
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: ::std::fmt::Arguments) -> ::std::io::Result<()> {
		self.0.write_fmt(fmt)
	}
}


impl<T: Write> Drop for FlushWrite<T> {
	#[inline(always)]
	fn drop(&mut self) {
		let _e = self.0.flush();
	}
}
