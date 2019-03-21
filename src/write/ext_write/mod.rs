
use ::std::io::Write;


mod std;
pub use self::std::*;

///The trait extends the capabilities of the standard Write.
pub trait ExtWrite<'a>: Write {
	type LockWrite: Write + 'a;

	///Blocking the output stream.
	fn lock(&'a self) -> Self::LockWrite;

	///Alternative method of blocking the output stream using the closure.
	#[inline(always)]
	fn lock_fn<F: FnOnce(Self::LockWrite) -> R, R>(&'a self, f: F) -> R {
		f(self.lock())
	}
}


///The trait extends the capabilities of the standard Write.
impl<'l, 'a, L, W> ExtWrite<'a> for &'l L
	where 
	L: ExtWrite<'a, LockWrite = W>, 
	W: 'a + Write,
	Self: Write,

{
	type LockWrite = W;

	
	///Blocking the output stream.
	#[inline(always)]
	fn lock(&'a self) -> Self::LockWrite {
		L::lock(self)
	}

	///Alternative method of blocking the output stream using the closure.
	#[inline(always)]
	fn lock_fn<F: FnOnce(Self::LockWrite) -> R, R>(&'a self, f: F) -> R {
		L::lock_fn(self, f)
	}
}

///The trait extends the capabilities of the standard Write.
impl<'l, 'a, L, W> ExtWrite<'a> for &'l mut L
	where 
	L: ExtWrite<'a, LockWrite = W>, 
	W: 'a + Write,
	Self: Write,

{
	type LockWrite = W;

	
	///Blocking the output stream.
	#[inline(always)]
	fn lock(&'a self) -> Self::LockWrite {
		L::lock(self)
	}

	///Alternative method of blocking the output stream using the closure.
	#[inline(always)]
	fn lock_fn<F: FnOnce(Self::LockWrite) -> R, R>(&'a self, f: F) -> R {
		L::lock_fn(self, f)
	}
}