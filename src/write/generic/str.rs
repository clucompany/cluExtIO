
use std::fmt;
use std::io;

pub trait WriteStr<'a> {
	type Ok;
	type Err;
	
	fn write_str(&mut self, s: &'a str) -> Result<Self::Ok, Self::Err>;
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&'a str]) -> Result<(), Self::Err> {
		for str in arr {
			let _e = self.write_str(str)?;
		}
		Ok( () )
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, _all_size: usize, arr: &'l [&'a str]) -> Result<(), Self::Err> {
		self.write_str_array(arr)
	}
}


/*impl<'a, 's> fmt::Write for &'s mut dyn WriteStr<'a, (), fmt::Error> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		WriteStr::<'a, (), fmt::Error>::write_str(self, s)
	}
}*/

impl<'a, 's, T, OK, ERR> WriteStr<'a> for &'s mut T where T: WriteStr<'a, Ok=OK, Err=ERR> {
	type Ok = T::Ok;
	type Err = T::Err;
	
	#[inline(always)]
	fn write_str(&mut self, s: &'a str) -> Result<Self::Ok, Self::Err> {
		T::write_str(self, s)
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&'a str]) -> Result<(), Self::Err> {
		T::write_str_array(self, arr)
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, _all_size: usize, arr: &'l [&'a str]) -> Result<(), Self::Err> {
		T::write_str_lenarray(self, _all_size, arr)
	}
}

impl<'a, 's, OK, ERR> WriteStr<'a> for &'s mut dyn WriteStr<'a, Ok=OK, Err=ERR> {
	type Ok = OK;
	type Err = ERR;
	
	#[inline(always)]
	fn write_str(&mut self, s: &'a str) -> Result<Self::Ok, Self::Err> {
		WriteStr::write_str(*self, s)
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&'a str]) -> Result<(), Self::Err> {
		WriteStr::write_str_array(*self, arr)
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, _all_size: usize, arr: &'l [&'a str]) -> Result<(), Self::Err> {
		WriteStr::write_str_lenarray(*self, _all_size, arr)
	}
}



impl<'a> WriteStr<'a> for Vec<u8> {
	type Ok = ();
	type Err = ();
	
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

impl<'a> WriteStr<'a> for Vec<&'a str> {
	type Ok = ();
	type Err = ();
	
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




impl<'a> WriteStr<'a> for String {
	type Ok = ();
	type Err = ();
	
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





impl<'a, 's> WriteStr<'a> for dyn fmt::Write + 's {
	type Ok = ();
	type Err = fmt::Error;
	
	#[inline(always)]
	fn write_str(&mut self, s: &'a str) -> Result<(), fmt::Error> {
		fmt::Write::write_str(self, s)
	}
}
impl<'a, 's> WriteStr<'a> for dyn io::Write + 's {
	type Ok = usize;
	type Err = io::Error;
	
	#[inline(always)]
	fn write_str(&mut self, s: &'a str) -> Result<usize, io::Error> {
		io::Write::write(self, s.as_bytes())
	}
}


