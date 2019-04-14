
use std::io;

pub type WriteFlushDefault = WriteFlush<Err = io::Error>;

pub trait WriteFlush {
	type Err;
	
	fn flush(&mut self) -> Result<(), Self::Err>;
}

impl<'s, ERR> WriteFlush for Box<dyn WriteFlush<Err = ERR> + 's> {
	type Err = ERR;
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), Self::Err> {
		(**self).flush()
	}
}

impl<'l, 's, ERR> WriteFlush for &'l mut (dyn WriteFlush<Err = ERR> + 's) {
	type Err = ERR;
	
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

/*impl<'l, 's> WriteFlush for &'l (dyn io::Write + 's) {
	type Err = io::Error;
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), Self::Err> {
		(*self).flush()
	}
}*/

