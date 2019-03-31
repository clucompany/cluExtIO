
use std::io::StderrLock;
use std::io::Stderr;
use std::io::Stdout;
use std::io::StdoutLock;

///The trait extends the capabilities of the standard Write.
pub trait LockWrite<'a> {
	type LockResult: 'a;

	///Blocking the output stream.
	fn lock(&'a self) -> Self::LockResult;

	///Alternative method of blocking the output stream using the closure.
	#[inline(always)]
	fn lock_fn<F: FnOnce(Self::LockResult) -> R, R>(&'a self, f: F) -> R {
		f(self.lock())
	}
}


///The trait extends the capabilities of the standard Write.
impl<'l, 'a, T, W> LockWrite<'a> for &'l T
	where 
	T: LockWrite<'a, LockResult = W>, 
	W: 'a,
{
	type LockResult = W;

	
	///Blocking the output stream.
	#[inline(always)]
	fn lock(&'a self) -> Self::LockResult {
		T::lock(self)
	}

	///Alternative method of blocking the output stream using the closure.
	#[inline(always)]
	fn lock_fn<F: FnOnce(Self::LockResult) -> R, R>(&'a self, f: F) -> R {
		T::lock_fn(self, f)
	}
}

///The trait extends the capabilities of the standard Write.
impl<'l, 'a, T, W> LockWrite<'a> for &'l mut T
	where 
	T: LockWrite<'a, LockResult = W>, 
	W: 'a,
{
	type LockResult = W;

	
	///Blocking the output stream.
	#[inline(always)]
	fn lock(&'a self) -> Self::LockResult {
		T::lock(self)
	}

	///Alternative method of blocking the output stream using the closure.
	#[inline(always)]
	fn lock_fn<F: FnOnce(Self::LockResult) -> R, R>(&'a self, f: F) -> R {
		T::lock_fn(self, f)
	}
}


impl<'a> LockWrite<'a> for Stdout {
	type LockResult = StdoutLock<'a>;

	#[inline(always)]
	fn lock(&'a self) -> Self::LockResult {
		self.lock()
	}
}

impl<'a> LockWrite<'a> for Stderr {
	type LockResult = StderrLock<'a>;

	#[inline(always)]
	fn lock(&'a self) -> Self::LockResult {
		self.lock()
	}
}
