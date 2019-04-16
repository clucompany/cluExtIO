
use std::marker::PhantomData;
use crate::LockWrite;
use std::io;
use std::fmt;
use std::io::Write;

///Immutability `Trait Write`.
pub trait ImmutWrite<'a> {
	type Err;
	
	fn write<'l>(&'a self, buf: &'l [u8]) -> Result<usize, Self::Err>;
	
	fn flush(&'a self) -> Result<(), Self::Err>;
	fn write_all<'l>(&'a self, buf: &'l [u8]) -> Result<(), Self::Err>;
	fn write_fmt<'l>(&'a self, fmt: fmt::Arguments<'l>) -> Result<(), Self::Err>;
}

impl<'a> ImmutWrite<'a> for () {
	type Err = ();
	
	#[inline]
	fn write<'l>(&'a self, _buf: &'l [u8]) -> Result<usize, Self::Err> {
		Ok( 0 )	
	}
	
	#[inline(always)]
	fn flush(&'a self) -> Result<(), Self::Err> {
		Ok( () )	
	}
	
	#[inline(always)]
	fn write_all<'l>(&'a self, _buf: &'l [u8]) -> Result<(), Self::Err> {
		Ok( () )	
	}
	
	#[inline(always)]
	fn write_fmt<'l>(&'a self, _fmt: fmt::Arguments<'l>) -> Result<(), Self::Err> {
		Ok( () )	
	}
}


impl<'a, E> ImmutWrite<'a> for ((), PhantomData<E>) {
	type Err = E;
	
	#[inline]
	fn write<'l>(&'a self, _buf: &'l [u8]) -> Result<usize, Self::Err> {
		Ok( 0 )	
	}
	
	#[inline(always)]
	fn flush(&'a self) -> Result<(), Self::Err> {
		Ok( () )	
	}
	
	#[inline(always)]
	fn write_all<'l>(&'a self, _buf: &'l [u8]) -> Result<(), Self::Err> {
		Ok( () )	
	}
	
	#[inline(always)]
	fn write_fmt<'l>(&'a self, _fmt: fmt::Arguments<'l>) -> Result<(), Self::Err> {
		Ok( () )	
	}
}


impl<'a, 't, E: 't> ImmutWrite<'a> for E where E: LockWrite<'a>, E::LockResult : io::Write {
	type Err = io::Error;
	
	fn write<'l>(&'a self, buf: &'l [u8]) -> Result<usize, Self::Err> {
		self.lock().write(buf)
	}

	fn flush(&'a self) -> Result<(), Self::Err> {
		self.lock().flush()
	}

	fn write_all<'l>(&'a self, buf: &'l [u8]) -> Result<(), Self::Err> {
		self.lock().write_all(buf)
	}

	fn write_fmt<'l>(&'a self, fmt: fmt::Arguments<'l>) -> Result<(), Self::Err> {
		self.lock().write_fmt(fmt)
	}
}

