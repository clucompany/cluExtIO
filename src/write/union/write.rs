
use crate::write::ext_write::ExtWrite;

use std::io::Write;
use std::io;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UnionWrite<W: Write, W2: Write>{
	write_left: W, 
	write_right: W2,
}

impl<W: Write, W2: Write> UnionWrite<W, W2> {
	#[inline]
	pub const fn new(out: W, out2: W2) -> Self {
		Self {
			write_left: out,
			write_right: out2,	
		}
	}
}

impl<W: Write, W2: Write> Write for UnionWrite<W, W2> {
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let e = self.write_left.write(buf);
		let e2 = self.write_right.write(buf);
		
		match e {
			Ok(ref size_0) => {
				match e2 {
					Ok(ref size_2) if size_0 >= size_2 => return Ok(*size_0),
					Ok(ref size_2) if size_0 < size_2 => return Ok(*size_2),
					err => return err,
				}
			},
			err => return err,
		}
	}
	#[inline(always)]
	fn flush(&mut self) -> io::Result<()> {
		let e = self.write_left.flush();
		let e2 = self.write_right.flush();

		if let Err(_) = e {
			return e;
		}
		e2
	}

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
		let e = self.write_left.write_all(buf);
		let e2 = self.write_right.write_all(buf);

		if let Err(_) = e {
			return e;
		}
		e2
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> {
		let e = self.write_left.write_fmt(fmt);
		let e2 = self.write_right.write_fmt(fmt);

		if let Err(_) = e {
			return e;
		}
		e2
	}
}

impl<'a, W: ExtWrite<'a>, W2: ExtWrite<'a>> ExtWrite<'a> for UnionWrite<W, W2> {
	type LockWrite = UnionWrite<W::LockWrite, W2::LockWrite>;

	fn lock(&'a self) -> Self::LockWrite {
		Self::LockWrite::new(self.write_left.lock(), self.write_right.lock())
	}
}

