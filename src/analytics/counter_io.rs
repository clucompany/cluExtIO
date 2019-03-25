
use crate::WriteFmt;
use std::ops::DerefMut;
use std::ops::Deref;
use crate::analytics::Counter;
use crate::analytics::NumOperations;
use crate::analytics::ByteCounter;
use std::fmt::Arguments;
use std::io;

#[derive(Debug)]
pub struct CounterIOWrite<W> where W: io::Write {
	io_write:		W,	
	
	write:		ByteCounter<usize, usize, usize, usize, usize>,
	flush:		NumOperations<usize, usize>,
	write_all:		ByteCounter<usize, usize, usize, usize, usize>,
	write_fmt:		NumOperations<usize, usize>,
	by_ref:		Counter<usize>,
}

impl<W> CounterIOWrite<W> where W: io::Write {
	pub fn new(w: W) -> Self {
		Self {
			io_write: w,
			
			write:	Default::default(),
			flush:	Default::default(),
			write_all:	Default::default(),
			write_fmt:	Default::default(),
			by_ref:	Default::default(),
		}	
	}
	
	pub fn is_not_empty(&self) -> (
		Option<&ByteCounter<usize, usize, usize, usize, usize>>,
		Option<&NumOperations<usize, usize>>,
		Option<&ByteCounter<usize, usize, usize, usize, usize>>,
		Option<&NumOperations<usize, usize>>,
		Option<&Counter<usize>>,
	) {
		(
			self.write.is_not_empty(),
			self.flush.is_not_empty(),
			self.write_all.is_not_empty(),
			self.write_fmt.is_not_empty(),
			
			self.by_ref.is_not_empty(),
		)
	}
	
	
	pub fn write_stat<WF: WriteFmt<E>, E>(&self, mut w: WF) -> Result<(), E> {
		
		if !self.write.is_empty() {
			writeln!(w, 
"fn write: ByteCounter {{
	all_byte:			{},
	
	successfully:			{},
	successfully_byte:		{},
	
	error:				{},
	error_byte:			{},
}}",
				self.write.all_byte, 
				
				self.write.successfully,
				self.write.successfully_byte,
				
				self.write.error,
				self.write.error_byte
			)?;
		}
		
		if !self.flush.is_empty() {
			writeln!(w, 
"fn flush: NumOperations {{
	ok:				{},
	err:				{},
}}",
				self.flush.ok,
				self.flush.error,
			)?;
		}
		
		if !self.write_all.is_empty() {
			writeln!(w, 
"fn write_all: ByteCounter {{
	all_byte:			{},
	
	successfully:			{},
	successfully_byte:		{},
	
	error:				{},
	error_byte:			{},
}}",
				self.write_all.all_byte, 
				
				self.write_all.successfully,
				self.write_all.successfully_byte,
				
				self.write_all.error,
				self.write_all.error_byte
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
		
		if !self.by_ref.is_empty() {
			writeln!(w, 
"fn by_ref: usize 				{}",
				self.by_ref.value
			)?;
		}
		
		Ok( () )
	}
}

impl<W> io::Write for CounterIOWrite<W> where W: io::Write {
	fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
		let len = buf.len();
		self.write.push_all_bytes(len);
		
		let result = self.io_write.write(buf);
		match result {
			Ok(ref _a) => self.write.push_successfully_byte(len),
			Err(ref _e) => self.write.push_error_byte(len),
		}
		result
	}
	
	fn flush(&mut self) -> Result<(), io::Error> {
		let result = self.io_write.flush();
		match result {
			Ok(ref _a) => self.flush.push_ok(),
			Err(ref _e) => self.flush.push_error(),
		}
		result
	}
	
	fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
		let len = buf.len();
		self.write_all.push_all_bytes(len);
		
		let result = self.io_write.write_all(buf);
		match result {
			Ok(ref _a) => self.write_all.push_successfully_byte(len),
			Err(ref _e) => self.write_all.push_error_byte(len),
		}
		
		result
	}
	
	fn write_fmt(&mut self, fmt: Arguments) -> Result<(), io::Error> {
		let result = self.io_write.write_fmt(fmt);
		match result {
			Ok(ref _a) => self.write_fmt.push_ok(),
			Err(ref _e) => self.write_fmt.push_error(),
		}
		result
	}
	
	#[inline]
	fn by_ref(&mut self) -> &mut Self {
		self.by_ref.push_value(1);
		self
	}
}

impl<W> From<W> for CounterIOWrite<W> where W: io::Write {
	#[inline(always)]
	fn from(a: W) -> Self {
		Self::new(a)
	}
}


impl<W> Deref for CounterIOWrite<W> where W: io::Write {
	type Target = W;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.io_write	
	}
}

impl<W> DerefMut for CounterIOWrite<W> where W: io::Write {
	#[inline(always)]
	fn deref_mut(&mut self)	-> &mut Self::Target {
		&mut self.io_write	
	}
}