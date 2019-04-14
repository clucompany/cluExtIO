
use crate::LockWrite;
use std::io::Write;
use std::io;
use std::fmt;

///Implementing the `Union Write` constructor for` Write`.
pub trait ConstUnionWrite where Self: Sized {
	#[inline(always)]
	fn union<B>(self, b: B) -> UnionWrite<Self, B> { 
		UnionWrite::new(self, b)
	}
}

impl<T> ConstUnionWrite for T where Self: Sized {}



#[derive(Debug, Clone)]
pub struct UnionWrite<W, W2> {
	write_left: W, 
	write_right: W2,
}

impl<W, W2> UnionWrite<W, W2> {
	#[inline]
	pub const fn new(out: W, out2: W2) -> Self {
		Self {
			write_left: out,
			write_right: out2,
		}
	}
	
	#[inline(always)]
	pub const fn as_left(&self) -> &W {
		&self.write_left
	}
	
	#[inline(always)]
	pub const fn as_right(&self) -> &W2 {
		&self.write_right
	}
}

impl<A, B> From<(A, B)> for UnionWrite<A, B> {
	#[inline(always)]
	fn from((a, b): (A, B)) -> Self {
		Self::new(a, b)
	}
}

impl<W, W2> Write for UnionWrite<W, W2> where W: io::Write, W2: io::Write {
	fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
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

	fn flush(&mut self) -> Result<(), io::Error> {
		let e = self.write_left.flush();
		let e2 = self.write_right.flush();

		if e.is_err() {
			return e;
		}
		e2
	}

	fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
		let e = self.write_left.write_all(buf);
		let e2 = self.write_right.write_all(buf);

		if e.is_err() {
			return e;
		}
		e2
	}

	fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), io::Error> {
		let e = self.write_left.write_fmt(fmt);
		let e2 = self.write_right.write_fmt(fmt);

		if e.is_err() {
			return e;
		}
		e2
	}
}



impl<W, W2> fmt::Write for UnionWrite<W, W2> where W: fmt::Write, W2: fmt::Write {
	fn write_fmt(&mut self, args: fmt::Arguments) -> Result<(), fmt::Error> {
		let e = self.write_left.write_fmt(args);
		let e2 = self.write_right.write_fmt(args);

		if e.is_err() {
			return e;
		}
		e2
	}
	
	fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
		let e = self.write_left.write_char(c);
		let e2 = self.write_right.write_char(c);

		if e.is_err() {
			return e;
		}
		e2
	}
	
	fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
		let e = self.write_left.write_str(s);
		let e2 = self.write_right.write_str(s);

		if e.is_err() {
			return e;
		}
		e2
	}
}




impl<'a, W, W2> LockWrite<'a> for UnionWrite<W, W2> where W: LockWrite<'a>, W2: LockWrite<'a> {
	type LockResult = UnionWrite<W::LockResult, W2::LockResult>;

	fn lock(&'a self) -> Self::LockResult {
		Self::LockResult::new(self.write_left.lock(), self.write_right.lock())
	}
}

