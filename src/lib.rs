/*

  This Library represents DLL, which can be called from ThinBASIC.

  The purpose of this code to give idea how to setup Rust to talk to ThinBASIC.

*/

extern crate libc;	// Needed for string interop

use libc::{c_char};
use std::ffi::CStr;

fn main()
{

}

// Numeric data types
#[no_mangle]
pub extern fn rsSumTwoSingles(a: f32, b: f32) -> f32 {
	a + b
}

#[no_mangle]
pub extern fn rsSumTwoDoubles(a: f64, b: f64) -> f64 {
	a + b
}

// String data types
#[no_mangle]
pub extern fn rsPrintASCIIZ(input: *const c_char){
	println!("---");
	let c_str = unsafe { CStr::from_ptr(input) };
	let input_text = c_str.to_str();
	println!("{:?}", input_text);
	println!("---");
}
