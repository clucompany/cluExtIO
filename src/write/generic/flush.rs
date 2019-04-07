
use std::io;

pub trait WriteFlush {
	type Err;
	
	fn flush(&mut self) -> Result<(), Self::Err>;
}

impl<'l, 's, E> WriteFlush for &'l (dyn WriteFlush<Err = E> + 's) {
	type Err = E;
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), E> {
		(self as &mut dyn WriteFlush<Err = Self::Err>).flush()
	}
}

impl<'l, 's, E> WriteFlush for &'l mut (dyn WriteFlush<Err = E> + 's) {
	type Err = E;
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), Self::Err> {
		(self as &mut dyn WriteFlush<Err = Self::Err>).flush()
	}
}

impl<'s, T> WriteFlush for T where T: io::Write + 's {
	type Err = io::Error;
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), Self::Err> {
		T::flush(self)
	}
}
