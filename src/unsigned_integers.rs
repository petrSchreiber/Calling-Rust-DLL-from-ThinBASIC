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
