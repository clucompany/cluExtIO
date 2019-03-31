
use std::io;

pub trait FlushWrite<E> {
	fn flush(&mut self) -> Result<(), E>;
}

impl<'l, 's, E> FlushWrite<E> for &'l (dyn FlushWrite<E> + 's) {
	#[inline(always)]
	fn flush(&mut self) -> Result<(), E> {
		(self as &mut dyn FlushWrite<E>).flush()
	}
}

impl<'l, 's, E> FlushWrite<E> for &'l mut (dyn FlushWrite<E> + 's) {
	#[inline(always)]
	fn flush(&mut self) -> Result<(), E> {
		(self as &mut dyn FlushWrite<E>).flush()
	}
}

impl<'s, T> FlushWrite<io::Error> for T where T: io::Write + 's {
	#[inline(always)]
	fn flush(&mut self) -> Result<(), io::Error> {
		T::flush(self)
	}
}
