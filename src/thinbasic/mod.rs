// Cargo for DLL support
extern crate libloading;
extern crate ascii;
extern crate winapi;

// Adjusting the signatures for our use
extern "system" {
    pub fn SysAllocStringByteLen(
        psz: *const u8,
        len: usize,
        ) -> *const u8;

    pub fn SysFreeString(
        bstrString: *const u8
    );

    pub fn SysStringByteLen(
        bstr: *const u8
    ) -> u32;    
}

// Custom ThinBasic string
pub struct TBString(*const u8);

// For creating from Rust str
impl<'a> From<&'a str> for TBString {
    
    fn from(str_text: &'a str) -> Self {
        unsafe
        {
            let ascii_str = ascii::AsciiStr::from_ascii(str_text).unwrap();
            let byte_slice = ascii_str.as_bytes();
            let ptr = SysAllocStringByteLen(&byte_slice[0], ascii_str.len());

            TBString(ptr)
        }
    }
}

// For releasing
impl Drop for TBString {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe
        {
            SysFreeString(self.0);
        }
    }
}

// Convenient, custom functions
impl TBString
{
    pub fn len(&self) -> u32 {
        unsafe {
            SysStringByteLen(self.0)
        }
    }
    
    pub fn to_string<'v>(&self) -> String {
        unsafe {
            let len = self.len();            
            let slice: &[u8] = ::std::slice::from_raw_parts(self.0, len as usize);

            String::from(::std::str::from_utf8(slice).unwrap())
        }
    }      
}
