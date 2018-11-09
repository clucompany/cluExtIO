
use write::ext_write::ExtWrite;
use std::io;
use std::fmt;
use std::io::Write;

///Unchangeable `Trait Write`.
pub trait NotChanWrite<'a> {
     fn write(&'a self, buf: &[u8]) -> io::Result<usize>;
     fn flush(&'a self) -> io::Result<()>;
     fn write_all(&'a self, buf: &[u8]) -> io::Result<()>;
     fn write_fmt(&'a self, fmt: fmt::Arguments) -> io::Result<()>;
}


impl<'a, E: ExtWrite<'a>> NotChanWrite<'a> for E {
     #[inline(always)]
     fn write(&'a self, buf: &[u8]) -> io::Result<usize> {
          self.lock().write(buf)
     }

     #[inline(always)]
     fn flush(&'a self) -> io::Result<()> {
          self.lock().flush()
     }

     #[inline(always)]
     fn write_all(&'a self, buf: &[u8]) -> io::Result<()> {
          self.lock().write_all(buf)
     }

     #[inline(always)]
     fn write_fmt(&'a self, fmt: fmt::Arguments) -> io::Result<()> { 
          self.lock().write_fmt(fmt)
     }
}


