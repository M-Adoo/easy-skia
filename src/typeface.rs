use std::ffi::CString;

use bindings::*;

pub struct Typeface {
    pub(crate) raw_pointer: *mut sk_typeface_t,
}

impl Typeface {
    pub fn new_from_file(path: &str, index: i32) -> Option<Typeface> {
        let c_str = CString::new(path).unwrap();
        let raw_pointer = unsafe { sk_typeface_create_from_file(c_str.as_ptr(), index) };

        if raw_pointer.is_null() {
            return None;
        }

        Some(Typeface { raw_pointer })
    }
}

impl Drop for Typeface {
    fn drop(&mut self) {
        unsafe { sk_typeface_unref(self.raw_pointer) };
    }
}
