

use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PhantomWriteStr<Ok, Err> {
	_pp: PhantomData<Ok>,
	_ppp: PhantomData<Err>,
}

impl<Ok, Err> PhantomWriteStr<Ok, Err> {
	#[inline(always)]
	pub const fn new() -> Self {
		Self {
			_pp: PhantomData,
			_ppp: PhantomData,
		}	
	}	
}

impl<Ok, Err> From<()> for PhantomWriteStr<Ok, Err> {
	#[inline(always)]
	fn from(_a: ()) -> Self {
		Self::new()
	}
}


impl<Ok, Err> Default for PhantomWriteStr<Ok, Err> {
	#[inline(always)]
	fn default() -> Self {
		Self::new()
	}
}




#[derive(Debug, Clone)]
pub struct PhantomLockWrite<'a, A, B> {
	_p:	PhantomData<&'a ()>,
	_pp:	PhantomData<A>,
	_ppp:	PhantomData<B>,
}

impl<'a, A, B> PhantomLockWrite<'a, A, B> {
	#[inline(always)]
	pub const fn new() -> Self {
		Self {
			_p:	PhantomData,
			_pp:	PhantomData,
			_ppp:	PhantomData,
		}	
	}	
}

impl<'a, A, B> From<()> for PhantomLockWrite<'a, A, B> {
	#[inline(always)]
	fn from(_a: ()) -> Self {
		Self::new()
	}
}


impl<'a, A, B> Default for PhantomLockWrite<'a, A, B> {
	#[inline(always)]
	fn default() -> Self {
		Self::new()
	}
}