

use cluExtIO::generic::WriteFmt;
use cluExtIO::generic::FmtORIoErr;

fn main() -> Result<(), FmtORIoErr> {
	let mut str = String::new();
	let mut vec = Vec::new();
	
	check_fmt(&mut str)?;
	check_fmt(&mut vec)?;
	
	println!("str: {:?}", str);
	println!("vec: {:?}", vec);
	
	Ok( () )
}


pub fn check_fmt<WF: WriteFmt<E>, E>(mut a: WF) -> Result<(), E> {
	a.write_fmt(format_args!("{}", 12))
}
