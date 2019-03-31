

use std::fmt::Arguments;
use std::fmt;
use std::io;
use std::ops::DerefMut;

pub trait WriteFmt<E> {
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), E>;
}

impl<'a, E> WriteFmt<E> for &'a mut WriteFmt<E> {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), E> {
		WriteFmt::write_fmt(*self, fmt)
	}
}

impl<E> WriteFmt<E> for Box<WriteFmt<E>> {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), E> {
		(**self).write_fmt(fmt)
	}
}



//FMT
impl<'a> WriteFmt<fmt::Error> for dyn fmt::Write + 'a {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), fmt::Error> {
		fmt::Write::write_fmt(self, fmt)
	}
}

impl<'a> WriteFmt<fmt::Error> for Box<dyn fmt::Write + 'a> {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), fmt::Error> {
		fmt::Write::write_fmt(self.deref_mut(), fmt)
	}
}


impl<'a, 'l> WriteFmt<fmt::Error> for &'l mut (dyn fmt::Write + 'a) {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), fmt::Error> {
		fmt::Write::write_fmt(self, fmt)	
	}
}

impl<'a, 'l, T> WriteFmt<fmt::Error> for &'l mut T where T: fmt::Write + 'a {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), fmt::Error> {
		fmt::Write::write_fmt(self, fmt)	
	}
}





impl<'a> WriteFmt<io::Error> for dyn io::Write + 'a {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), io::Error> {
		io::Write::write_fmt(self, fmt)	
	}
}

impl<'a> WriteFmt<io::Error> for Box<dyn io::Write + 'a> {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), io::Error> {
		io::Write::write_fmt(self.deref_mut(), fmt)	
	}
}

impl<'a, 'l> WriteFmt<io::Error> for &'l mut (dyn io::Write + 'a) {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), io::Error> {
		io::Write::write_fmt(self, fmt)	
	}
}

impl<'a, 'l, T> WriteFmt<io::Error> for &'l mut T where T: io::Write + 'a {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), io::Error> {
		io::Write::write_fmt(self, fmt)
	}
}


//Vec<u8>
impl WriteFmt<io::Error> for Vec<u8> {
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

impl FmtORIoErr {
	pub fn is_fmt(&self) -> bool {
		match self {
			FmtORIoErr::FMT(_) => true,
			_ => false,
		}	
	}
	
	pub fn is_io(&self) -> bool {
		match self {
			FmtORIoErr::IO(_) => true,
			_ => false,
		}	
	}
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


impl WriteFmt<FmtORIoErr> for dyn io::Write {
	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), FmtORIoErr> {
		match io::Write::write_fmt(self, fmt) {
			Ok(_a) => Ok( () ),
			Err(e) => Err(From::from(e)),
		}
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
