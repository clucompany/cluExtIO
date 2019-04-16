
use std::error::Error;
use crate::write::generic::WriteStr;
use crate::LockWrite;
use crate::write::generic::WriteFlush;
use std::ops::DerefMut;
use std::ops::Deref;
use std::marker::PhantomData;
use std::io;
use std::fmt;

#[derive(Debug)]
pub struct FlushFn<T, E, F> where T: WriteFlush<Err = E>, E: Error, F: FnMut(&mut T) {
	write: T,
	_p: PhantomData<E>,
	r#fn: F,
}

impl<T, E, F> FlushFn<T, E, F> where T: WriteFlush<Err = E>, E: Error, F: FnMut(&mut T) {
	#[inline]
	pub const fn new(write: T, r#fn: F) -> Self {
		Self {
			write: write,
			_p: PhantomData,
			r#fn: r#fn,
		}
	}
	
	pub fn flush(&mut self) -> Result<(), E> {
		(self.r#fn)(&mut self.write);
		self.write.flush()
	}
}

/*Conflict
impl<T, E, F> WriteFlush for FlushFn<T, E, F> where T: WriteFlush<Err = E>, E: Error, F: FnMut(&mut T) {
	type Err = E;
	
	
}
*/

impl<T, E, F> From<(T, F)> for FlushFn<T, E, F> where T: WriteFlush<Err = E>, E: Error, F: FnMut(&mut T) {
	#[inline(always)]
	fn from((t, f): (T, F)) -> Self {
		Self::new(t, f)
	}
}

impl<T, E, F> Deref for FlushFn<T, E, F> where T: WriteFlush<Err = E>, E: Error, F: FnMut(&mut T) {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &T {
		&self.write
	}
}

impl<T, E, F> DerefMut for FlushFn<T, E, F> where T: WriteFlush<Err = E>, E: Error, F: FnMut(&mut T) {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.write
	}
}

/*
impl<T, E> WriteFlush for FlushFn<T, E> where T: WriteFlush<Err = E> {
	type Err = E;
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), Self::Err> {
		self.write.flush()	
	}
}*/


impl<T, F> io::Write for FlushFn<T, io::Error, F> where T: WriteFlush<Err = io::Error> + io::Write, F: FnMut(&mut T) {
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
		self.write.write(buf)
	}
	
	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
		self.write.write_all(buf)
	}
	
	#[inline(always)]
	fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), io::Error> {
		self.write.write_fmt(fmt)
	}
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), io::Error> {
		Self::flush(self)
	}
}


impl<T, F, E> fmt::Write for FlushFn<T, E, F> where T: WriteFlush<Err = E> + fmt::Write, E: Error, F: FnMut(&mut T), E: Error {
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
		self.write.write_str(s)
	}
	
	#[inline(always)]
	fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
		self.write.write_char(c)
	}
	
	#[inline(always)]
	fn write_fmt(self: &mut Self, args: fmt::Arguments) -> Result<(), fmt::Error> {
		self.write.write_fmt(args)
	}
}

impl<'a, T, OK, E, F> WriteStr for FlushFn<T, E, F> where T: WriteFlush<Err = E> + WriteStr<Ok = OK, Err = E>, E: Error, F: FnMut(&mut T) {
	type Ok = OK;
	type Err = E;
	
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> Result<Self::Ok, Self::Err> {
		self.write.write_str(s)
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&str]) -> Result<(), Self::Err> {
		self.write.write_str_array(arr)
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, all_size: usize, arr: &'l [&str]) -> Result<(), Self::Err> {
		self.write.write_str_lenarray(all_size, arr)
	}
}





impl<'a, T, E, F> LockWrite<'a> for FlushFn<T, E, F> 
	where T: LockWrite<'a> + WriteFlush<Err = E>, E: Error, F: FnMut(&mut T) {
	
	type LockResult = T::LockResult;
	
	#[inline(always)]
	fn lock(&'a self) -> Self::LockResult {
		self.write.lock()
	}
}
