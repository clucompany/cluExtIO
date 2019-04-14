
use crate::write::generic::WriteStr;
use crate::LockWrite;
use crate::write::generic::WriteFlush;
use std::ops::DerefMut;
use std::ops::Deref;
use std::marker::PhantomData;
use std::io;
use std::fmt;

pub type FlushIOWrite = DropWriteFlush<io::Write, io::Error>;

#[derive(Debug)]
pub struct DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	write: T,
	_p: PhantomData<E>,
}

impl<T, E> DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	#[inline]
	pub const fn new(write: T) -> Self {
		Self {
			write: write,
			_p: PhantomData,
		}
	}
}

impl<T, E> From<T> for DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)
	}
}

impl<T, E> Deref for DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &T {
		&self.write
	}
}

impl<T, E> DerefMut for DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.write
	}
}

/*
impl<T, E> WriteFlush for DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	type Err = E;
	
	#[inline(always)]
	fn flush(&mut self) -> Result<(), Self::Err> {
		self.write.flush()	
	}
}*/


impl<T, E> io::Write for DropWriteFlush<T, E> where T: WriteFlush<Err = E> + io::Write {
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
		io::Write::flush(&mut self.write)
	}
}


impl<T, E> fmt::Write for DropWriteFlush<T, E> where T: WriteFlush<Err = E> + fmt::Write {
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

impl<'a, T, OK, E> WriteStr for DropWriteFlush<T, E> where T: WriteFlush<Err = E> + WriteStr<Ok = OK, Err = E> {
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





impl<'a, T> LockWrite<'a> for DropWriteFlush<T, io::Error> 
	where T: LockWrite<'a> + WriteFlush<Err = io::Error> + io::Write {
	
	type LockResult = T::LockResult;
	
	#[inline(always)]
	fn lock(&'a self) -> Self::LockResult {
		self.write.lock()
	}
}



impl<T, E> Drop for DropWriteFlush<T, E> where T: WriteFlush<Err = E> {
	fn drop(&mut self) {
		let _e = self.write.flush();
	}
}
