
use crate::write::ext_write::ExtWrite;

use std::fmt::Debug;
use std::io::Write;
use std::io;
use std::fmt;

pub struct UnionWrite<W: Write, W2: Write>(W, W2);

impl<W: Write, W2: Write> UnionWrite<W, W2> {
	#[inline]
	pub fn new(out: W, out2: W2) -> Self {
		UnionWrite(out, out2)
	}
}


impl<W: Write, W2: Write> Debug for UnionWrite<W, W2> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.pad("UnionWrite { .. }")
	}
}

impl<W: Write, W2: Write> Write for UnionWrite<W, W2> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          let e = self.0.write(buf);
          let e2 = self.1.write(buf);
          
          match e {
               Ok(ref size_0) => {
                    match e2 {
                         Ok(ref size_2) if size_0 >= size_2 => return Ok(*size_0),
                         Ok(ref size_2) if size_0 < size_2 => return Ok(*size_2),
                         err => return err,
                    }
               },
               err => return err,
          }
     }
     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          let e = self.0.flush();
          let e2 = self.1.flush();
		if let Err(_) = e {
               return e;
          }
          e2
     }

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
		let e = self.0.write_all(buf);
          let e2 = self.1.write_all(buf);

		if let Err(_) = e {
               return e;
          }
          e2
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> {
          let e = self.0.write_fmt(fmt);
          let e2 = self.1.write_fmt(fmt);

		if let Err(_) = e {
               return e;
          }
          e2
	}
}

impl<W: Write + Clone, W2: Write + Clone> Clone for UnionWrite<W, W2> {
     fn clone(&self) -> Self {
          Self::new(self.0.clone(), self.1.clone())
     }
}

impl<'a, W: ExtWrite<'a>, W2: ExtWrite<'a>> ExtWrite<'a> for UnionWrite<W, W2> {
     type LockWrite = UnionWrite<W::LockWrite, W2::LockWrite>;

     fn lock(&'a self) -> Self::LockWrite {
          UnionWrite::new(self.0.lock(), self.1.lock())
     }
}

impl<'a, W: 'static + 'a + ExtWrite<'a>, W2: 'static + 'a + ExtWrite<'a>> Into<Box<Write + 'a>> for UnionWrite<W, W2> {
     fn into(self) -> Box<Write> {
          Box::new(self)
     }
}

