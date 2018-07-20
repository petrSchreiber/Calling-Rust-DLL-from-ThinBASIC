use thinbasic::TBString;

// STRING
#[no_mangle]
pub extern fn rsReturnString() -> TBString {
    return TBString::from("Hello ThinBASIC, I am Rust STRING")
}
