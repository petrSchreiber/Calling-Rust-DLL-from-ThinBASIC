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
