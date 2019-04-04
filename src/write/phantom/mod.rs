

use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PhantomWriteStr<'a, Ok, Err> {
	_p: PhantomData<&'a ()>,
	_pp: PhantomData<Ok>,
	_ppp: PhantomData<Err>,
}

impl<'a, Ok, Err> PhantomWriteStr<'a, Ok, Err> {
	#[inline(always)]
	pub const fn new() -> Self {
		Self {
			_p: PhantomData,
			_pp: PhantomData,
			_ppp: PhantomData,	
		}	
	}	
}