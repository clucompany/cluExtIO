
use std::fmt;
use std::io;

pub trait WriteStr {
	type Ok;
	type Err;
	
	fn write_str(&mut self, s: &str) -> Result<Self::Ok, Self::Err>;
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&str]) -> Result<(), Self::Err> {
		for str in arr {
			let _e = self.write_str(str)?;
		}
		Ok( () )
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, _all_size: usize, arr: &'l [&str]) -> Result<(), Self::Err> {
		self.write_str_array(arr)
	}
}


impl<'s, T, OK, ERR> WriteStr for &'s mut T where T: WriteStr<Ok=OK, Err=ERR> {
	type Ok = T::Ok;
	type Err = T::Err;
	
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> Result<Self::Ok, Self::Err> {
		T::write_str(self, s)
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&str]) -> Result<(), Self::Err> {
		T::write_str_array(self, arr)
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, _all_size: usize, arr: &'l [&str]) -> Result<(), Self::Err> {
		T::write_str_lenarray(self, _all_size, arr)
	}
}

impl<'s, OK, ERR> WriteStr for &'s mut dyn WriteStr<Ok=OK, Err=ERR> {
	type Ok = OK;
	type Err = ERR;
	
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> Result<Self::Ok, Self::Err> {
		WriteStr::write_str(*self, s)
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&str]) -> Result<(), Self::Err> {
		WriteStr::write_str_array(*self, arr)
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, _all_size: usize, arr: &'l [&str]) -> Result<(), Self::Err> {
		WriteStr::write_str_lenarray(*self, _all_size, arr)
	}
}



impl WriteStr for Vec<u8> {
	type Ok = ();
	type Err = ();
	
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> Result<(), ()> {
		self.extend_from_slice(s.as_bytes());
		Ok( () )
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&str]) -> Result<(), ()> {
		for a in arr.into_iter() {
			self.extend_from_slice(a.as_bytes());
		}
		
		Ok( () )
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, all_size: usize, arr: &'l [&str]) -> Result<(), ()> {
		self.reserve(all_size);
		for a in arr.into_iter() {
			self.extend_from_slice(a.as_bytes());
		}
		Ok( () )
	}
}




impl WriteStr for String {
	type Ok = ();
	type Err = ();
	
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> Result<(), ()> {
		self.push_str(s);
		
		Ok( () )
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&str]) -> Result<(), ()> {
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





impl<'s> WriteStr for dyn fmt::Write + 's {
	type Ok = ();
	type Err = fmt::Error;
	
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
		fmt::Write::write_str(self, s)
	}
}

impl<'s> WriteStr for dyn io::Write + 's {
	type Ok = usize;
	type Err = io::Error;
	
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> Result<usize, io::Error> {
		io::Write::write(self, s.as_bytes())
	}
}

//
impl<'s> fmt::Write for &'s mut dyn WriteStr<Ok=(), Err=fmt::Error> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		WriteStr::write_str(self, s)
	}
}