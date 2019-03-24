
use std::ops::DerefMut;
use std::ops::Deref;
use crate::analytics::NumOperations;
use crate::analytics::ByteCounter;
use std::fmt::Arguments;
use std::io;
use std::fmt;

#[derive(Debug)]
pub struct CounterFMTWrite<W> where W: fmt::Write {
	fmt_write:		W,	
	
	write_str:		ByteCounter<usize, usize, usize, usize, usize>,
	write_char:		ByteCounter<usize, usize, usize, usize, usize>,
	write_fmt:		NumOperations<usize, usize>,
	
}

impl<W> CounterFMTWrite<W> where W: fmt::Write {
	pub fn new(w: W) -> Self {
		Self {
			fmt_write: w,
			
			write_str:		Default::default(),
			write_char:		Default::default(),
			write_fmt:		Default::default(),
		}	
	}
	
	pub fn is_not_empty(&self) -> (
		Option<&ByteCounter<usize, usize, usize, usize, usize>>,
		Option<&ByteCounter<usize, usize, usize, usize, usize>>,
		Option<&NumOperations<usize, usize>>,
	) {
		(
			self.write_str.is_not_empty(),
			self.write_char.is_not_empty(),
			self.write_fmt.is_not_empty(),
		)
	}
	
	
	pub fn write_stat<WR: io::Write>(&self, mut w: WR) -> Result<(), io::Error> {
		
		if !self.write_str.is_empty() {
			writeln!(w, 
"fn write_str: ByteCounter {{
	all_byte:			{},
	
	successfully:			{},
	successfully_byte:		{},
	
	error:				{},
	error_byte:			{},
}}",
				self.write_str.all_byte, 
				
				self.write_str.successfully,
				self.write_str.successfully_byte,
				
				self.write_str.error,
				self.write_str.error_byte
			)?;
		}
		
		if !self.write_char.is_empty() {
			writeln!(w, 
"fn write_char: ByteCounter {{
	all_byte:			{},
	
	successfully:			{},
	successfully_byte:		{},
	
	error:				{},
	error_byte:			{},
}}",
				self.write_char.all_byte, 
				
				self.write_char.successfully,
				self.write_char.successfully_byte,
				
				self.write_char.error,
				self.write_char.error_byte
			)?;
		}
		
		if !self.write_fmt.is_empty() {
			writeln!(w, 
"fn write_fmt: NumOperations {{
	ok:				{},
	err:				{},
}}",
				self.write_fmt.ok,
				self.write_fmt.error,
			)?;
		}
		
		Ok( () )
	}
}

impl<W> fmt::Write for CounterFMTWrite<W> where W: fmt::Write {
	fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
		let len = s.len();
		self.write_str.push_all_bytes(len);
		
		let result = self.fmt_write.write_str(s);
		match result {
			Ok(ref _a) => self.write_str.push_successfully_byte(len),
			Err(ref _e) => self.write_str.push_error_byte(len),
		}
		result
	}

	fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
		let len = c.len_utf8();
		self.write_char.push_all_bytes(len);
		
		let result = self.fmt_write.write_char(c);
		match result {
			Ok(ref _a) => self.write_char.push_successfully_byte(len),
			Err(ref _e) => self.write_char.push_error_byte(len),
		}
		result
	}
	
	fn write_fmt(&mut self, args: Arguments) -> Result<(), fmt::Error> { 		
		let result = self.fmt_write.write_fmt(args);
		match result {
			Ok(ref _a) => self.write_fmt.push_ok(),
			Err(ref _e) => self.write_fmt.push_error(),
		}
		result
	}
}

impl<W> From<W> for CounterFMTWrite<W> where W: fmt::Write {
	#[inline(always)]
	fn from(a: W) -> Self {
		Self::new(a)
	}
}


impl<W> Deref for CounterFMTWrite<W> where W: fmt::Write {
	type Target = W;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.fmt_write	
	}
}

impl<W> DerefMut for CounterFMTWrite<W> where W: fmt::Write {
	#[inline(always)]
	fn deref_mut(&mut self)	-> &mut Self::Target {
		&mut self.fmt_write	
	}
}