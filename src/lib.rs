/*

  This Library represents DLL, which can be called from ThinBASIC.

  The purpose of this code to give idea how to setup Rust to talk to ThinBASIC.

*/

extern crate libc;    // Needed for string interop

use libc::{c_char};
use std::ffi::CStr;
use std::ffi::CString;

/*

  Numeric data types - no surprises here

*/

// Byte or UInt8
#[no_mangle]
pub extern fn rsSumTwoBytes(a: u8, b: u8) -> u8 {
    a + b
}

// Word or UInt16
#[no_mangle]
pub extern fn rsSumTwoWords(a: u16, b: u16) -> u16 {
    a + b
}

// Dword or UInt32
#[no_mangle]
pub extern fn rsSumTwoDwords(a: u32, b: u32) -> u32 {
    a + b
}

// Integer or Int16
#[no_mangle]
pub extern fn rsSumTwoIntegers(a: i16, b: i16) -> i16 {
    a + b
}

// Long or Int32
#[no_mangle]
pub extern fn rsSumTwoLongs(a: i32, b: i32) -> i32 {
    a + b
}

// Quad or Int64
#[no_mangle]
pub extern fn rsSumTwoQuads(a: i64, b: i64) -> i64 {
    a + b
}

// Single or Float32
#[no_mangle]
pub extern fn rsSumTwoSingles(a: f32, b: f32) -> f32 {
    a + b
}

// Double or Float64
#[no_mangle]
pub extern fn rsSumTwoDoubles(a: f64, b: f64) -> f64 {
    a + b
}

// Extended or Number, that is, 80 bits of precision, are not supported in Rust :(


/*

  String data types - here things start to get interesting :)

*/  

// ASCIIZ
#[no_mangle]
pub extern fn rsPrintAsciiz(input: *const c_char) {
    let c_str = unsafe {
        assert!(!input.is_null());                                        // NUL pointers not allowed in Rust

        CStr::from_ptr(input)
    };

    let input_text = c_str.to_str();
    println!("{}", input_text.unwrap());
}

#[no_mangle]
pub extern fn rsReturnAsciizPtr() -> *const c_char {
    let message = CString::new("Hello ThinBASIC, I am Rust").unwrap();    // Create string
    let p_message = message.as_ptr();                                     // Retrieve its pointer
    std::mem::forget(message);                                            // Make Rust forget the string, to prevent it from going outta scope

    p_message
}
