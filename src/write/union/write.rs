


use write::ext_write::ExtWrite;
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
     #[inline]
	pub fn boxed(out: W, out2: W2) -> Box<Self> {
		Box::new(Self::new(out, out2))
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
               Ok(s) => {
                    match e2 {
                         Ok(s2) => return Ok({
                              if s2 >= s {
                                   s
                              }else {
                                   s2
                              }
                         }),
                         a => return a,
                    }
               },
               a => return a,
          }
     }
     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          let e = self.0.flush();
          let e2 = self.1.flush();
		if let Err(e) = e {
               return Err(e);
          }
          e2
     }

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
		let e = self.0.write_all(buf);
          let e2 = self.1.write_all(buf);
		if let Err(e) = e {
               return Err(e);
          }
          e2
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> {
          let e = self.0.write_fmt(fmt);
          let e2 = self.1.write_fmt(fmt);
		if let Err(e) = e {
               return Err(e);
          }
          e2
	}
}

impl<W: Write + Clone, W2: Write + Clone> Clone for UnionWrite<W, W2> {
     fn clone(&self) -> Self {
          Self::new(self.0.clone(), self.1.clone())
     }
}

impl<'a, W: Write + ExtWrite<'a>, W2: Write + ExtWrite<'a>> ExtWrite<'a> for UnionWrite<W, W2> {
     type Lock = UnionWrite<W::Lock, W2::Lock>;

     fn lock(&'a self) -> Self::Lock {
          UnionWrite::new(self.0.lock(), self.1.lock())
     }
}
