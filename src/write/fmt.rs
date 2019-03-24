

use std::fmt::Arguments;
use std::fmt;
use std::io;

pub trait WriteFmt<E> {
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), E>;
}

impl<'a> WriteFmt<fmt::Error> for dyn fmt::Write + 'a {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), fmt::Error> {
		fmt::Write::write_fmt(self, fmt)
	}
}

impl<'a, T> WriteFmt<fmt::Error> for &mut T where T: fmt::Write + 'a {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), fmt::Error> {
		fmt::Write::write_fmt(self, fmt)	
	}
}


impl<'a> WriteFmt<FmtORIoErr> for dyn fmt::Write + 'a {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), FmtORIoErr> {
		match fmt::Write::write_fmt(self, fmt) {
			Ok(_) => Ok(()),
			Err(e) => Err(From::from(e)),
		}
	}
}


impl<'a> WriteFmt<io::Error> for dyn io::Write + 'a {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), io::Error> {
		io::Write::write_fmt(self, fmt)	
	}
}

impl WriteFmt<FmtORIoErr> for dyn io::Write {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), FmtORIoErr> {
		match io::Write::write_fmt(self, fmt) {
			Ok(_a) => Ok( () ),
			Err(e) => Err(From::from(e)),
		}
	}
}

impl<T> WriteFmt<io::Error> for &mut T where T: io::Write {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), io::Error> {
		io::Write::write_fmt(self, fmt)
	}
}

#[derive(Debug)]
pub enum FmtORIoErr {
	FMT(fmt::Error),
	IO(io::Error),
}

impl From<fmt::Error> for FmtORIoErr {
	#[inline(always)]
	fn from(a: fmt::Error) -> Self {
		FmtORIoErr::FMT(a)
	}
}

impl From<io::Error> for FmtORIoErr {
	#[inline(always)]
	fn from(a: io::Error) -> Self {
		FmtORIoErr::IO(a)
	}
}
