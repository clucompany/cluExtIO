
use std::error::Error;
use std::io;

pub type WriteFlushDefault = WriteFlush<Err = io::Error>;

pub trait WriteFlush {
	type Err: Error;
	
	fn flush(&mut self) -> Result<(), Self::Err>;
}

impl<'s, ERR> WriteFlush for Box<dyn WriteFlush<Err = ERR> + 's> where ERR: Error {
	type Err = ERR;
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), Self::Err> {
		(**self).flush()
	}
}

impl<'l, 's, ERR> WriteFlush for &'l mut (dyn WriteFlush<Err = ERR> + 's) where ERR: Error {
	type Err = ERR;
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), Self::Err> {
		(**self).flush()
	}
}

impl<'s> WriteFlush for dyn io::Write + 's /*T where T: io::Write + 's*/ {
	type Err = io::Error;
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), Self::Err> {
		io::Write::flush(self)
	}
}

impl<'s, T> WriteFlush for T where T: io::Write + 's {
	type Err = io::Error;
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), Self::Err> {
		io::Write::flush(self)
	}
}


/*impl<'s, 'b, F, E> WriteFlush for (F, dyn WriteFlush<Err = E> + 's) where F: FnMut() + 'b, E: Error {
	type Err = E;
	
	fn flush(&mut self) -> Result<(), Self::Err> {
		self.0();
		self.1.flush()
	}
}
*/


/*impl<'l, 's> WriteFlush for &'l (dyn io::Write + 's) {
	type Err = io::Error;
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), Self::Err> {
		(*self).flush()
	}
}*/

