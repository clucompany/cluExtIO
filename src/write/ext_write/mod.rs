
use ::std::io::Write;


mod std;
pub use self::std::*;



///The trait extends the capabilities of the standard Write.
pub trait ExtWrite<'a>: Write {
    type Lock: Write + 'a;

    ///Blocking the output stream.
    fn lock(&'a self) -> Self::Lock;

    ///Alternative method of blocking the output stream using the closure.
    #[inline]
    fn lock_fn<F: FnMut(Self::Lock) -> R, R>(&'a self, mut f: F) -> R {
        f(self.lock())
    }
}

///The trait extends the capabilities of the standard Write.
impl<'a, 'l, L: ExtWrite<'a, Lock = W>, W: 'a +  Write> ExtWrite<'a> for &'l L where Self: Write + 'a {
    type Lock = W;
    ///Blocking the output stream.
    fn lock(&'a self) -> Self::Lock {
        (**self).lock()
    }

    ///Alternative method of blocking the output stream using the closure.
    #[inline]
    fn lock_fn<F: FnMut(Self::Lock) -> R, R>(&'a self, f: F) -> R {
        (**self).lock_fn(f)
    }
}

///The trait extends the capabilities of the standard Write.
impl<'a, 'l, L: ExtWrite<'a, Lock = W>, W: 'a +  Write> ExtWrite<'a> for &'l mut L where Self: Write + 'a {
    type Lock = W;
    ///Blocking the output stream.
    fn lock(&'a self) -> Self::Lock {
        (**self).lock()
    }

    ///Alternative method of blocking the output stream using the closure.
    #[inline]
    fn lock_fn<F: FnMut(Self::Lock) -> R, R>(&'a self, f: F) -> R {
        (**self).lock_fn(f)
    }
}