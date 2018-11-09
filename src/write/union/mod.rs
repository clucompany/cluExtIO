

mod write;
pub use self::write::*;

use std::io::Write;

///Implementing the `Union Write` constructor for` Write`.
pub trait UnionWriteConst: Write {
     #[inline]
     fn union<B: Write>(self, b: B) -> UnionWrite<Self, B> where Self: Sized { 
          UnionWrite::new(self, b)
     }
}

impl<T: Write + Sized> UnionWriteConst for T {}

/* conflict:(
impl<T: Write + Sized> Add for T {
     type Output = UnionWrite<Self, T>;

     #[inline]
     fn add(self, b: Self) -> Self::Output {
          UnionWrite::new(self, b)
     }
}
*/