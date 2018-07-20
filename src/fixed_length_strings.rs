extern crate libc;    // Needed for string interop
use self::libc::{c_char};

use std::ffi::CStr;
use std::ffi::CString;

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
    ::std::mem::forget(message);                                          // Make Rust forget the string, to prevent it from going outta scope

    p_message
}
