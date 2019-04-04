

use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PhantomWriteStr<Ok, Err> {
	//_p: PhantomData<&'a ()>,
	_pp: PhantomData<Ok>,
	_ppp: PhantomData<Err>,
}

impl<Ok, Err> PhantomWriteStr<Ok, Err> {
	#[inline(always)]
	pub const fn new() -> Self {
		Self {
			//_p: PhantomData,
			_pp: PhantomData,
			_ppp: PhantomData,	
		}	
	}	
}

impl<Ok, Err> Default for PhantomWriteStr<Ok, Err> {
	#[inline(always)]
	fn default() -> Self {
		Self::new()	
	}	
}