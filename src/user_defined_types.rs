// User defined types
#[repr(C)]              // Ensures Rust will not change the order of fields
#[derive(Debug)]        // Allows to print the struct easily
pub struct RgbColor
{
    r: u8, // u8 = BYTE
    g: u8,
    b: u8
}

#[repr(C)]
#[derive(Debug)]
pub struct Vector3D
{
    x: f32, // f32 = SINGLE
    y: f32,
    z: f32
}

#[no_mangle]
pub extern fn rsPrintUdtRgbColor(color: &mut RgbColor) {
    println!("Nice color, ThinBASIC: {:?}", *color)
}

#[no_mangle]
pub extern fn rsMultiplyVector3D(vector_ptr: &mut Vector3D, multiplier: f32) {
    (*vector_ptr).x *= multiplier;
    (*vector_ptr).y *= multiplier;
    (*vector_ptr).z *= multiplier;
}

#[no_mangle]
pub extern fn rsMultiplyVector3DArray(vector_ptr: *mut Vector3D, count: i32, multiplier: f32) {
    
    // Converting pointer to so called slice, the _mut allows us to write
    let vector = unsafe { ::std::slice::from_raw_parts_mut(vector_ptr, count as usize) };

    // Writing new values
    for i in 0..count {
        vector[i as usize].x *= multiplier;
        vector[i as usize].y *= multiplier;
        vector[i as usize].z *= multiplier;
    }
}
