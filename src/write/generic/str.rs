
use std::fmt;
use std::io;

pub trait WriteStr<'a, Ok, Err> {
	fn write_str(&mut self, s: &'a str) -> Result<Ok, Err>;
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&'a str]) -> Result<(), Err> {
		for str in arr {
			let _e = self.write_str(str)?;
		}
		
		Ok( () )
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, _all_size: usize, arr: &'l [&'a str]) -> Result<(), Err> {
		self.write_str_array(arr)
	}
}


/*impl<'a, 's> fmt::Write for &'s mut dyn WriteStr<'a, (), fmt::Error> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		WriteStr::<'a, (), fmt::Error>::write_str(self, s)
	}
}*/


impl<'a, 's, Ok, Err> WriteStr<'a, Ok, Err> for &'s mut dyn WriteStr<'a, Ok, Err> {
	#[inline(always)]
	fn write_str(&mut self, s: &'a str) -> Result<Ok, Err> {
		WriteStr::write_str(*self, s)
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&'a str]) -> Result<(), Err> {
		WriteStr::write_str_array(*self, arr)
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, _all_size: usize, arr: &'l [&'a str]) -> Result<(), Err> {
		WriteStr::write_str_lenarray(*self, _all_size, arr)
	}
}



impl<'a> WriteStr<'a, (), ()> for Vec<u8> {
	#[inline(always)]
	fn write_str(&mut self, s: &'a str) -> Result<(), ()> {
		self.extend_from_slice(s.as_bytes());
		Ok( () )
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&'a str]) -> Result<(), ()> {
		for a in arr.into_iter() {
			self.extend_from_slice(a.as_bytes());
		}
		
		Ok( () )
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, all_size: usize, arr: &'l [&'a str]) -> Result<(), ()> {
		self.reserve(all_size);
		for a in arr.into_iter() {
			self.extend_from_slice(a.as_bytes());
		}
		Ok( () )
	}
}

impl<'a> WriteStr<'a, (), ()> for Vec<&'a str> {
	#[inline(always)]
	fn write_str(&mut self, s: &'a str) -> Result<(), ()> {
		self.push(s);
		Ok( () )
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&'a str]) -> Result<(), ()> {
		for a in arr.into_iter() {
			self.push(a);
		}
		
		Ok( () )
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, all_size: usize, arr: &'l [&'a str]) -> Result<(), ()> {
		self.reserve(all_size);
		for a in arr.into_iter() {
			self.push(a);
		}
		Ok( () )
	}
}




impl<'a> WriteStr<'a, (), ()> for String {
	#[inline(always)]
	fn write_str(&mut self, s: &'a str) -> Result<(), ()> {
		self.push_str(s);
		Ok( () )
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&'a str]) -> Result<(), ()> {
		for a in arr.into_iter() {
			self.push_str(a);
		}
		
		Ok( () )
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, all_size: usize, arr: &'l [&str]) -> Result<(), ()> {
		self.reserve(all_size);
		for a in arr.into_iter() {
			self.push_str(a);
		}
		Ok( () )
	}
}





impl<'a, 's> WriteStr<'a, (), fmt::Error> for dyn fmt::Write + 's {
	#[inline(always)]
	fn write_str(&mut self, s: &'a str) -> Result<(), fmt::Error> {
		fmt::Write::write_str(self, s)
	}
}
impl<'a, 's> WriteStr<'a, usize, io::Error> for dyn io::Write + 's {
	#[inline(always)]
	fn write_str(&mut self, s: &'a str) -> Result<usize, io::Error> {
		io::Write::write(self, s.as_bytes())
	}
}



impl<'a, 's> WriteStr<'a, (), ErrWriteStr> for dyn fmt::Write + 's {
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> Result<(), ErrWriteStr> {
		if let Err(e) = fmt::Write::write_str(self, s) {
			return Err(e.into());	
		}
		Ok( () )
	}
}

impl<'a, 's> WriteStr<'a, (), ErrWriteStr> for dyn io::Write + 's {
	#[inline(always)]
	fn write_str(&mut self, s: &'a str) -> Result<(), ErrWriteStr> {
		if let Err(e) = io::Write::write(self, s.as_bytes()) {
			return Err(e.into());
		}
		Ok( () )
	}
}



#[derive(Debug)]
pub enum ErrWriteStr {
	IO(io::Error),
	FMT(fmt::Error),
	None,
}


impl From<()> for ErrWriteStr {
	#[inline(always)]
	fn from(_r: ()) -> Self {
		ErrWriteStr::None
	}
}

impl From<io::Error> for ErrWriteStr {
	#[inline(always)]
	fn from(r: io::Error) -> Self {
		ErrWriteStr::IO(r)
	}
}

impl From<fmt::Error> for ErrWriteStr {
	#[inline(always)]
	fn from(r: fmt::Error) -> Self {
		ErrWriteStr::FMT(r)	
	}
}
