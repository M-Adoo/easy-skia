use std::ffi::CString;

use bindings::*;

pub struct Typeface {
    pub(crate) native_pointer: *mut sk_typeface_t,
}

impl Typeface {
    pub fn new_from_file(path: &str, index: i32) -> Option<Typeface> {
        let c_str = CString::new(path).unwrap();
        let native_pointer = unsafe { sk_typeface_create_from_file(c_str.as_ptr(), index) };

        if native_pointer.is_null() {
            return None;
        }

        Some(Typeface { native_pointer })
    }
}

impl Drop for Typeface {
    fn drop(&mut self) {
        unsafe { sk_typeface_unref(self.native_pointer) };
    }
}
