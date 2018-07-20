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
