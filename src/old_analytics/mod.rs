
mod counter_io;
pub use self::counter_io::*;

mod counter_fmt;
pub use self::counter_fmt::*;

use std::ops::AddAssign;

#[derive(Debug, Default)]
pub struct NumOperations<A, B> {
	pub ok: A,
	pub error: B,
}

impl<A, B> NumOperations<A, B> {
	#[inline]
	pub const fn new(a: A, b: B) -> Self {
		Self {
			ok: a,
			error: b,	
		}
	}
	
	pub fn push_ok(&mut self) where A: AddAssign<usize> {
		self.ok += 1;	
	}
	
	pub fn push_error(&mut self) where B: AddAssign<usize> {
		self.error += 1;	
	}
	
	#[inline(always)]
	pub const fn count_ok(&self) -> &A {
		&self.ok	
	}
	
	#[inline(always)]
	pub const fn count_err(&self) -> &B {
		&self.error
	}
}

impl<A, B> NumOperations<A, B> where A: PartialOrd<usize>, B: PartialOrd<usize> {
	pub fn is_empty(&self) -> bool {
		if self.ok > 0 || self.error > 0 {
			return false;
		}	
		true
	}
	
	pub fn is_not_empty(&self) -> Option<&Self> {
		match self.is_empty() {
			true => None,
			_ => Some(self)
		}	
	}
}


impl<A, B> From<(A, B)> for NumOperations<A, B> {
	#[inline(always)]
	fn from((a, b): (A, B)) -> Self {
		Self::new(a, b)	
	}
}


#[derive(Debug, Default)]
pub struct ByteCounter<A, B, C, D, F> {
	pub all_byte: A,
	
	
	pub successfully: B,
	pub successfully_byte: C,
	
	pub error: D,
	pub error_byte: F
}

impl<A, B, C, D, F> ByteCounter<A, B, C, D, F> {
	#[inline]
	pub const fn new(a: A, b: B, c: C, d: D, f: F) -> Self {
		Self {
			all_byte: a,
			successfully: b,
			successfully_byte: c,
			error: d,
			error_byte: f
		}
	}
	
	pub fn push_all_bytes(&mut self, a: usize) where A: AddAssign<usize> {
		self.all_byte += a;	
	}
	
	pub fn push_successfully_byte(&mut self, a: usize) where B: AddAssign<usize>, C: AddAssign<usize> {
		self.successfully += 1;
		self.successfully_byte += a;
	}
	
	pub fn push_error_byte(&mut self, a: usize) where D: AddAssign<usize>, F: AddAssign<usize> {
		self.error += 1;
		self.error_byte += a;	
	}
	
	#[inline(always)]
	pub const fn as_all_byte(&self) -> &A {
		&self.all_byte
	}
	
	#[inline(always)]
	pub const fn as_successfully(&self) -> &B {
		&self.successfully
	}
	
	#[inline(always)]
	pub const fn as_successfully_byte(&self) -> &C {
		&self.successfully_byte
	}
	
	#[inline(always)]
	pub const fn as_error(&self) -> &D {
		&self.error
	}
	
	#[inline(always)]
	pub const fn as_error_byte(&self) -> &F {
		&self.error_byte
	}
}

impl<A, B, C, D, F> ByteCounter<A, B, C, D, F>
	where 
	A: PartialOrd<usize>, B: PartialOrd<usize>,
	C: PartialOrd<usize>, D: PartialOrd<usize>,
	F: PartialOrd<usize>,
{
	pub fn is_empty(&self) -> bool {
		if self.all_byte > 0 || self.successfully > 0 || self.successfully_byte > 0 || self.error > 0 || self.error_byte > 0 {
			return false;
		}
		true
	}
	
	pub fn is_not_empty(&self) -> Option<&Self> {
		match self.is_empty() {
			true => None,
			_ => Some(self)	
		}	
	}
}


impl<A, B, C, D, F> From<(A, B, C, D, F)> for ByteCounter<A, B, C, D, F> {
	#[inline(always)]
	fn from((a, b, c, d, f): (A, B, C, D, F)) -> Self {
		Self::new(a, b, c, d, f)	
	}
}

#[derive(Debug, Default)]
pub struct Counter<A> {
	value: A	
}

impl<A> Counter<A> {
	#[inline]
	pub const fn new(a: A) -> Self {
		Self {
			value: a	
		}	
	}
	
	pub fn push_value(&mut self, a: usize) where A: AddAssign<usize> {
		self.value += a;	
	}
	
	#[inline(always)]
	pub const fn as_value(&self) -> &A {
		&self.value
	}
}

impl<A> Counter<A> where A: PartialOrd<usize> {
	pub fn is_empty(&self) -> bool {
		self.value == 0	
	}
	
	pub fn is_not_empty(&self) -> Option<&Self> {
		match self.is_empty() {
			true => None,
			_ => Some(self)	
		}	
	}
}

impl<A> From<A> for Counter<A> {
	#[inline(always)]
	fn from(a: A) -> Self {
		Self::new(a)	
	}
}