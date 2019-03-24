
mod write;
pub use self::write::*;

use std::io::Write;

///Implementing the `Union Write` constructor for` Write`.
pub trait UnionWriteConst: where Self: Write {
	#[inline]
	fn union<B: Write>(self, b: B) -> UnionWrite<Self, B> where Self: Sized { 
		UnionWrite::new(self, b)
	}
}

impl<T> UnionWriteConst for T where T: Write {}


impl<A, B> From<(A, B)> for UnionWrite<A, B> where A: Write, B: Write {
	#[inline(always)]
	fn from((a, b): (A, B)) -> Self {
		a.union(b)
	}
}
