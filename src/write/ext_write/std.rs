
use std::io::StderrLock;
use std::io::Stderr;
use write::ext_write::ExtWrite;
use std::io::StdoutLock;
use std::io::Stdout;

impl<'a> ExtWrite<'a> for Stdout {
     type Lock = StdoutLock<'a>;
     #[inline(always)]
     fn lock(&'a self) -> Self::Lock {
          self.lock()
     }
}

impl<'a> ExtWrite<'a> for Stderr {
     type Lock = StderrLock<'a>;
     #[inline(always)]
     fn lock(&'a self) -> Self::Lock {
          self.lock()
     }
}

