
use std::ops::DerefMut;
use std::ops::Deref;
use std::io;
use std::fmt;

#[derive(Debug)]
pub struct DropWriteArray<T, A> where T: io::Write, A: AsRef<[u8]> {
	write: T,
	array: A,
}

impl<T, A> DropWriteArray<T, A> where T: io::Write, A: AsRef<[u8]> {
	#[inline]
	pub const fn new(write: T, array: A) -> Self {
		Self {
			write: write,
			array: array,
		}
	}
}

impl<T, A> From<(T, A)> for DropWriteArray<T, A> where T: io::Write, A: AsRef<[u8]> {
	#[inline(always)]
	fn from((t, a): (T, A)) -> Self {
		Self::new(t, a)
	}
}

impl<T, A> Deref for DropWriteArray<T, A> where T: io::Write, A: AsRef<[u8]> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &T {
		&self.write
	}
}

impl<T, A> DerefMut for DropWriteArray<T, A> where T: io::Write, A: AsRef<[u8]> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.write
	}
}



impl<T, A> io::Write for DropWriteArray<T, A> where T: io::Write, A: AsRef<[u8]> {
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
		self.write.write(buf)	
	}
	
	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
		self.write.write_all(buf)
	}
	
	#[inline(always)]
	fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), io::Error> {
		self.write.write_fmt(fmt)
	}
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), io::Error> {
		io::Write::flush(&mut self.write)
	}
}


impl<T, A> Drop for DropWriteArray<T, A> where T: io::Write, A: AsRef<[u8]> {
	fn drop(&mut self) {
		let _e = self.write.write(self.array.as_ref());
	}
}
