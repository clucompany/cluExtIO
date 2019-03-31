
use std::io;

pub trait WriteFlush<E> {
	fn flush(&mut self) -> Result<(), E>;
}

impl<'l, 's, E> WriteFlush<E> for &'l (dyn WriteFlush<E> + 's) {
	#[inline(always)]
	fn flush(&mut self) -> Result<(), E> {
		(self as &mut dyn WriteFlush<E>).flush()
	}
}

impl<'l, 's, E> WriteFlush<E> for &'l mut (dyn WriteFlush<E> + 's) {
	#[inline(always)]
	fn flush(&mut self) -> Result<(), E> {
		(self as &mut dyn WriteFlush<E>).flush()
	}
}

impl<'s, T> WriteFlush<io::Error> for T where T: io::Write + 's {
	#[inline(always)]
	fn flush(&mut self) -> Result<(), io::Error> {
		T::flush(self)
	}
}
