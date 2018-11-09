
use std::sync::MutexGuard;
use std::io::Write;
use std::io;
use std::fmt;

#[derive(Debug)]
pub struct GuardWrite<'a, T: 'a +  Write>(MutexGuard<'a, T>);

impl<'a, T: Write> GuardWrite<'a, T> {
     #[inline]
     pub fn guard(t: MutexGuard<'a, T>) -> Self {
          GuardWrite(t)
     }

     #[inline]
     pub fn boxed(t: MutexGuard<'a, T>) -> Box<Self> {
          Box::new(Self::guard(t))
     }
}

impl<'a, T: 'a + Write> Write for GuardWrite<'a, T> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          self.0.write(buf)
     }

     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          self.0.flush()
     }

     #[inline(always)]
     fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
          self.0.write_all(buf)
     }

     #[inline(always)]
     fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> { 
          self.0.write_fmt(fmt)
     }
}
