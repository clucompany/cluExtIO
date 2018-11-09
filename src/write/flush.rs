
use std::io::Write;
use std::io;
use std::fmt;

#[derive(Debug)]
///An implementation of `Trait Write`, which calls the flush () method on drop.                                  
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
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.0.write(buf)
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()> {
		self.0.flush()
	}

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
		self.0.write_all(buf)
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> {
		self.0.write_fmt(fmt)
	}
}

impl<T: Write + Clone> Clone for FlushWrite<T> {
     #[inline]
     fn clone(&self) -> Self {
          Self::new(self.0.clone())
     }
}

impl<T: Write> Drop for FlushWrite<T> {
	#[inline(always)]
	fn drop(&mut self) {
		let _e = self.0.flush();
	}
}

