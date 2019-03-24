
//Doubt...

use std::fmt;
use std::io;

pub trait WriteStr {
	fn write_str(&mut self, s: &str) -> WriteStrResult;
}

impl<'a> WriteStr for dyn fmt::Write + 'a {
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> WriteStrResult {
		fmt::Write::write_str(self, s).into()
	}
}
impl<'a> WriteStr for dyn io::Write + 'a {
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> WriteStrResult {
		io::Write::write(self, s.as_bytes()).into()
	}
}


#[derive(Debug)]
pub enum WriteStrResult {
	IO(Result<usize, io::Error>),
	FMT(Result<(), fmt::Error>),
}

impl From<Result<usize, io::Error>> for WriteStrResult {
	#[inline(always)]
	fn from(r: Result<usize, io::Error>) -> Self {
		WriteStrResult::IO(r)	
	}
}

impl From<Result<(), fmt::Error>> for WriteStrResult {
	#[inline(always)]
	fn from(r: Result<(), fmt::Error>) -> Self {
		WriteStrResult::FMT(r)	
	}
}